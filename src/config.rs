use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

use color_eyre::eyre::{bail, ContextCompat};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::AppResult;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Configuration {
	pub previously_opened_files: Vec<PathBuf>,
	pub location: Option<PathBuf>,
}

impl Configuration {
	pub fn load_or_default() -> AppResult<Self> {
		let mut base = dirs::config_dir().context("No file was picked")?;
		fs::create_dir_all(
			{
				let mut base = base.clone();
				base.push("wt_ext_gui");
				base
			}
		)?;
		base.push("wt_ext_gui/config.toml");
		match fs::File::open(&base) {
			Ok(mut f) => {
				let mut buf = String::new();
				f.read_to_string(&mut buf)?;
				let mut s = toml::from_str::<Configuration>(&buf)?;
				s.location = Some(base);
				Ok(s)
			}
			Err(e) => {
				match e.kind() {
					ErrorKind::NotFound => {
						let mut f = fs::File::create(&base)?;
						let default = Self {
							previously_opened_files: vec![],
							location: Some(base),
						};
						f.write_all(toml::to_string_pretty(&default)?.as_bytes())?;
						Ok(default)
					}
					_ => {
						bail!(e);
					}
				}
			}
		}
	}
	pub fn save(&self) -> AppResult<()> {
		info!("{} {:?}", "Saving", self);
		fs::write(&self.location.clone().context("Attempted to save config with no location set")?, toml::to_string_pretty(&self)?)?;
		info!("Saved configuration");
		Ok(())
	}
}