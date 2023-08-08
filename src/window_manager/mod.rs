use eframe::egui;

pub mod home;

pub trait Window
{
	fn update(&mut self, ctx: &egui::Context);
	fn new() -> Self where Self: Sized;
}