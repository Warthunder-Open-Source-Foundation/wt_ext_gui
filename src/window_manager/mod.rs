use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use eframe::egui;
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
	pub fn view_with_path(path: PathBuf) -> Self {
		Self::ChangeTo(
			Window::View(
				View {
					opened_path: path,
					raw_view: None,
					active_task: Arc::new(RwLock::new(ActiveTask::Idle)),
					file_reader_thread: None,
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