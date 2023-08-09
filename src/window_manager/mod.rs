use eframe::egui;
use crate::App;
use crate::config::Configuration;

use crate::window_manager::home::Home;

pub mod home;


pub enum Window {
	Home(Home),
}

impl Window {
	pub fn render(&mut self, ctx: &egui::Context, app: &mut Configuration) {
		match self {
			Window::Home(home) => {
				home.show(ctx, app);
			}
		}
	}
}

impl Default for Window {
	fn default() -> Self {
		Self::Home(Home::default())
	}
}