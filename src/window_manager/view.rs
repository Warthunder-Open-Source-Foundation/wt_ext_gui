use std::path::PathBuf;
use eframe::egui::Context;
use crate::AppResult;
use crate::config::Configuration;
use crate::window_manager::WindowChange;

pub struct View {
	pub(crate) opened_path: PathBuf,
}

impl View {
	pub fn show(&mut self, ctx: &Context, app: &mut Configuration) -> AppResult<WindowChange> {
		Ok(WindowChange::LeaveUnchanged)
	}
}