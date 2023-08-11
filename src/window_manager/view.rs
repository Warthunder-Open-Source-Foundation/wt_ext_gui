use std::{fs, thread};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::JoinHandle;

use color_eyre::Report;
use eframe::egui;
use eframe::egui::{ProgressBar, ScrollArea};
use tracing::info;
use wt_blk::blk::BlkOutputFormat;
use wt_blk::vromf::unpacker::{File, VromfUnpacker};

use crate::AppResult;
use crate::config::Configuration;
use crate::window_manager::view::ActiveTask::UnpackingVromf;
use crate::window_manager::WindowChange;

pub struct View {
	/// Vromf path
	pub opened_path: PathBuf,

	/// Raw vromf as it was read off the disk
	pub raw_view: Option<Vec<u8>>,

	/// In-progress report for footer
	pub active_task: Arc<RwLock<ActiveTask>>,

	/// Thread handle that reads vromf from disk
	pub file_reader_thread: Option<JoinHandle<Result<Vec<u8>, Report>>>,

	///
	pub unpack_progress: Arc<(AtomicUsize, AtomicUsize)>,

	/// Final unpacked files
	pub unpacked_files: Option<Vec<File>>,
	pub unpacked_files_thread: Option<JoinHandle<Result<Vec<File>, Report>>>,

	pub(crate) icon: egui::TextureHandle,
}

impl View {
	pub fn show(&mut self, ctx: &egui::Context, app: &mut Configuration) -> AppResult<WindowChange> {
		self.read_file()?;
		self.parse_vromfs()?;
		egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
			ui.small(format!("{}", self.active_task.read().unwrap()));
			Ok::<_, Report>(())
		}).inner?;
		egui::CentralPanel::default().show(ctx, |ui| {
			match self.active_task.read().unwrap().deref() {
				UnpackingVromf { left, total } => {
					ui.add(
						ProgressBar::new(
							((*total - *left) as f32 / *total as f32)
						)
							.show_percentage()
							.text("Unpacking vromf")
					);
				}
				_ => {}
			}
			if let Some(files) = &self.unpacked_files {
				ScrollArea::vertical()
					.auto_shrink([false,false])
					.show(ui, |ui| {
					for file in files {
						ui.horizontal(|ui|{
							ui.image(&self.icon, self.icon.size_vec2());
							ui.label(file.0.to_string_lossy());
						});
					}
				});
			}
		});
		Ok(WindowChange::LeaveUnchanged)
	}

	fn read_file(&mut self) -> AppResult<()> {
		if self.raw_view.is_none() {
			if let Some(handle) = self.file_reader_thread.take() {
				if handle.is_finished() {
					info!("Reader thread has finished");
					self.raw_view = Some(
						handle
							.join()
							.unwrap()?
					);
					self.active_task
						.write()
						.unwrap()
						.idle();
				} else {
					self.file_reader_thread = Some(handle);
				}
			} else {
				let status = self.active_task.clone();
				let p = self.opened_path.clone();
				let t = std::thread::spawn(move || {
					*status.write().unwrap() = ActiveTask::ReadingVromfToMemory;
					let read = fs::read(p)?;
					info!("Read vromf to memory, size: {}", read.len());
					Ok::<_, Report>(read)
				});
				self.file_reader_thread = Some(t);
			}
		}
		Ok(())
	}
	fn parse_vromfs(&mut self) -> AppResult<()> {
		if self.unpacked_files.is_none() && let Some(vromf) = self.raw_view.clone() {
			*self.active_task.write().unwrap() = ActiveTask::UnpackingVromf { left: self.unpack_progress.0.load(Ordering::Relaxed), total: self.unpack_progress.1.load(Ordering::Relaxed) };
			if let Some(t) = self.unpacked_files_thread.take() {
				if t.is_finished() {
					self.unpacked_files = Some(t.join().unwrap()?);
					self.active_task.write().unwrap().idle();
				} else {
					self.unpacked_files_thread = Some(t);
				}
			} else {
				info!("Spawn vromf unpacking thread");
				let path = self.opened_path.clone();
				let progress = self.unpack_progress.clone();
				let t = thread::spawn(move || {
					let unpacker = VromfUnpacker::from_file((path, vromf))?;
					Ok(unpacker.unpack_all_with_progress(Some(BlkOutputFormat::BlkText), progress)?)
				});
				self.unpacked_files_thread = Some(t);
			}
		}
		Ok(())
	}
}

#[derive(Clone, Debug)]
pub enum ActiveTask {
	ReadingVromfToMemory,
	UnpackingVromf {
		left: usize,
		total: usize,
	},
	Idle,
}

impl Display for ActiveTask {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			ActiveTask::ReadingVromfToMemory => { "reading Vromf to memory".to_string() }
			ActiveTask::Idle => { "idle".to_string() }
			ActiveTask::UnpackingVromf { left, total } => format!("unpacking {total} files, {left} remaining"),
		})
	}
}

impl ActiveTask {
	pub fn idle(&mut self) {
		*self = Self::Idle;
	}
}