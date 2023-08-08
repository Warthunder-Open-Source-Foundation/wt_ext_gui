#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod render;
mod window_manager;

const APP_NAME: &str = "wt_ext_gui";

use crate::window_manager::home::Home;
use crate::window_manager::Window;

pub struct App {
    window: Box<dyn Window>,
}

impl App  {
    pub fn set_window(&mut self, window: Box<dyn Window>){
        self.window = window;
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: Box::new(Home::new()),
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
