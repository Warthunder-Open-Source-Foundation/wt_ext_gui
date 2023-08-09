#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::window_manager::Window;

mod render;
mod window_manager;

const APP_NAME: &str = "wt_ext_gui";

pub struct App {
	active_window: Window,
}

impl App {
	pub fn set_window(&mut self, window: Window) {
		self.active_window = window;
	}
}

impl Default for App {
	fn default() -> Self {
		Self {
			active_window: Window::default(),
		}
	}
}

fn main() -> Result<(), eframe::Error> {
	color_eyre::install().unwrap();

	let options = eframe::NativeOptions {
		..Default::default()
	};
	eframe::run_native(
		APP_NAME,
		options,
		Box::new(|_| Box::<App>::default()),
	)
}
