#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io;
use std::io::stdout;
use color_eyre::Report;
use crate::config::Configuration;
use crate::window_manager::Window;

mod render;
mod window_manager;
mod config;

const APP_NAME: &str = "wt_ext_gui";

type AppResult<T> = Result<T, Report>;

pub struct App {
	configuration: Configuration,
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
			configuration: Configuration::load_or_default().unwrap(),
			active_window: Window::default(),
		}
	}
}

fn main() -> Result<(), eframe::Error> {
	color_eyre::install().unwrap();
	tracing_subscriber::fmt()
		.with_writer(io::stdout)
		.init();

	let options = eframe::NativeOptions {
		..Default::default()
	};
	eframe::run_native(
		APP_NAME,
		options,
		Box::new(|_| Box::<App>::default()),
	)
}
