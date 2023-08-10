use std::path::PathBuf;
use color_eyre::Report;

use eframe::egui;
use eframe::egui::{Align, Context, Layout};

use crate::AppResult;
use crate::config::Configuration;
use crate::window_manager::{Window, WindowChange};
use crate::window_manager::view::View;

pub struct Home {
	selected_file: Option<PathBuf>,
}

impl Home {
	pub fn show(&mut self, ctx: &Context, app: &mut Configuration) -> AppResult<WindowChange> {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Home");
			ui.label(self.selected_file.clone().unwrap_or(Default::default()).to_string_lossy().to_string());
			let mut ret = ui.with_layout(
				Layout::top_down_justified(Align::Center)
					.with_cross_justify(false),
				|ui| {
					if ui.button("Select file to open").clicked() {
						self.selected_file = native_dialog::FileDialog::new()
							.add_filter("Vromf file", &["bin"])
							.set_location(&dirs::home_dir().unwrap())
							.show_open_single_file()?;
						if let Some(prev) = &self.selected_file {
							app.previously_opened_files.push(prev.to_owned());
							app.save()?;
							return Ok(WindowChange::ChangeTo(Window::View(View::default())));
						}
					};
					Ok::<_, Report>(WindowChange::LeaveUnchanged)
				},
			).inner?;
			ui.label("Recently opened");
			for previously_opened_file in &app.previously_opened_files {
				if ui.button(previously_opened_file.to_string_lossy().to_string()).clicked() {
					ret =  WindowChange::ChangeTo(Window::View(View::default()));
				}
			}
			Ok(ret)
		}).inner
	}
}

impl Default for Home {
	fn default() -> Self {
		Self {
			selected_file: None,
		}
	}
}
