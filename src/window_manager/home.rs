use eframe::egui;
use eframe::egui::Context;
use crate::window_manager::Window;

pub struct Home {

}

impl Window for Home {
	fn update(&mut self, ctx: &Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Home");
		});
	}

	fn new() -> Self where Self: Sized {
		Self {

		}
	}
}
