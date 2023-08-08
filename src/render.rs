use eframe::egui;
use crate::App;

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.window.update(ctx);
	}
}