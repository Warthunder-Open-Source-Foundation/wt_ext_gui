use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use eframe::egui;
use eframe::egui::{ColorImage, Context};
use image::{DynamicImage, GenericImageView, ImageFormat};
use image::imageops::{FilterType, resize, thumbnail};
use crate::{App, AppResult};
use crate::config::Configuration;

use crate::window_manager::home::Home;
use crate::window_manager::view::{ActiveTask, View};

pub mod home;
pub mod view;

pub enum WindowChange {
	ChangeTo(Window),
	LeaveUnchanged,
}

impl WindowChange {
	pub fn view_with_path(path: PathBuf, ctx: &Context) -> Self {
		let mut img = image::load_from_memory(&fs::read("/home/flareflo/Downloads/8750719.png").unwrap()).unwrap();
		img = thumbnail(&img,32,32).into();
		Self::ChangeTo(
			Window::View(
				View {
					opened_path: path,
					raw_view: None,
					active_task: Arc::new(RwLock::new(ActiveTask::Idle)),
					file_reader_thread: None,
					unpack_progress: Default::default(),
					unpacked_files: None,
					unpacked_files_thread: None,
					icon: ctx.load_texture("binary-icon", ColorImage::from_rgba_unmultiplied([img.width() as _, img.height() as _], img.as_bytes()), Default::default()),
				}
			)
		)
	}
}


pub enum Window {
	Home(Home),
	View(View),
}

impl Window {
	pub fn render(&mut self, ctx: &egui::Context, app: &mut Configuration) -> AppResult<WindowChange> {
		return Ok(match self {
			Window::Home(home) => {
				home.show(ctx, app)?
			}
			Window::View(view) => {
				view.show(ctx, app)?
			}
		})
	}
}

impl Default for Window {
	fn default() -> Self {
		Self::Home(Home::default())
	}
}