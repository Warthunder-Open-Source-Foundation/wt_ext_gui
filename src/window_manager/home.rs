use std::path::PathBuf;
use color_eyre::eyre::ContextCompat;
use color_eyre::Report;

use eframe::egui;
use eframe::egui::{Align, Context, Layout};
use crate::AppResult;
use crate::config::Configuration;

pub struct Home {
	selected_file: Option<PathBuf>,
}

impl Home {
	pub fn show(&mut self, ctx: &Context, app: &mut Configuration) -> AppResult<()> {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Home");
			ui.label(self.selected_file.clone().unwrap_or(Default::default()).to_string_lossy().to_string());
			ui.with_layout(
				Layout::top_down_justified(Align::Center)
					.with_cross_justify(false),
				|ui| {
					if ui.button("Select file to open").clicked() {
						self.selected_file = Some(
							native_dialog::FileDialog::new()
								.add_filter("Vromf file", &["bin"])
								.set_location(&dirs::home_dir().unwrap())
								.show_open_single_file()
								?.context("No file selected")?
						);
						if let Some(prev) = &self.selected_file {
							app.previously_opened_files.push(prev.to_owned());
							app.save()?;
						}
					};
					Ok::<(), Report>(())
				}
			).inner?;
			ui.label("Recently opened");
			for previously_opened_file in &app.previously_opened_files {
				let _ = ui.button(previously_opened_file.to_string_lossy().to_string());
			}
			Ok::<(), Report>(())
		}).inner?;
		Ok(())
	}
}

impl Default for Home {
	fn default() -> Self {
		Self {
			selected_file: None,
		}
	}
}
