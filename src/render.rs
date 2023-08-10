use eframe::egui;

use crate::App;
use crate::window_manager::WindowChange;

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let next = self.active_window.render(ctx, &mut self.configuration).unwrap();
		match next {
			WindowChange::ChangeTo(window) => {self.active_window = window}
			WindowChange::LeaveUnchanged => {}
		}
	}
}