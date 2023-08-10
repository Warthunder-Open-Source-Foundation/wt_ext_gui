use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use color_eyre::eyre::ContextCompat;

use color_eyre::Report;
use eframe::egui;
use tracing::info;

use crate::AppResult;
use crate::config::Configuration;
use crate::window_manager::WindowChange;

pub struct View {
	pub opened_path: PathBuf,
	pub raw_view: Option<Vec<u8>>,
	pub active_task: Arc<RwLock<ActiveTask>>,

	pub file_reader_thread: Option<JoinHandle<Result<Vec<u8>, Report>>>,
}

impl View {
	pub fn show(&mut self, ctx: &egui::Context, app: &mut Configuration) -> AppResult<WindowChange> {
		if self.raw_view.is_none() {
			if self.file_reader_thread.is_some() {
				let handle = self.file_reader_thread.take();
				if handle.as_ref().context("")?.is_finished() {
					info!("Reader thread has finished");
					self.raw_view = Some(self.file_reader_thread
						.take()
						.expect("infallible")
						.join()
						.unwrap()
						.unwrap()
					);
					self.active_task
						.write()
						.unwrap()
						.idle();
				} else {
					self.file_reader_thread = handle;
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

		egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
			ui.small(format!("Task: {}", self.active_task.read().expect("Whag")));
			Ok::<_, Report>(())
		}).inner?;
		egui::CentralPanel::default().show(ctx, |ui| {});
		Ok(WindowChange::LeaveUnchanged)
	}
}

#[derive(Clone, Debug)]
pub enum ActiveTask {
	ReadingVromfToMemory,
	Idle,
}

impl Display for ActiveTask {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			ActiveTask::ReadingVromfToMemory => { "reading Vromf to memory" }
			ActiveTask::Idle => { "idle" }
		})
	}
}

impl ActiveTask {
	pub fn idle(&mut self) {
		*self = Self::Idle;
	}
}