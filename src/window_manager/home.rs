use std::path::PathBuf;

use eframe::egui;
use eframe::egui::{Align, Context, Layout};

pub struct Home {
	selected_file: Option<PathBuf>,
}

impl Home {
	pub fn show(&mut self, ctx: &Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Home");
			ui.label(self.selected_file.clone().unwrap_or(Default::default()).to_string_lossy().to_string());
			ui.with_layout(
				Layout::top_down_justified(Align::Center)
					.with_cross_justify(false),
				|ui| if ui.button("Select file to open").clicked() {
					self.selected_file = Some(
						native_dialog::FileDialog::new()
							.add_filter("Vromf file", &["bin"])
							.show_open_single_file()
							.unwrap()
							.unwrap()
					);
				},
			);
		});
	}
}

impl Default for Home {
	fn default() -> Self {
		Self {
			selected_file: None,
		}
	}
}
