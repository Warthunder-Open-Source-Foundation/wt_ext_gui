use eframe::egui;

use crate::window_manager::home::Home;

pub mod home;


pub enum Window {
	Home(Home),
}

impl Window {
	pub fn render(&mut self, ctx: &egui::Context) {
		match self {
			Window::Home(home) => {
				home.show(ctx);
			}
		}
	}
}

impl Default for Window {
	fn default() -> Self {
		Self::Home(Home::default())
	}
}