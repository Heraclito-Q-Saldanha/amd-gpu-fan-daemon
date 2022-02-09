pub mod init;
pub mod load;
pub mod push;

use custom_error::custom_error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub use crate::init::init;
pub use crate::load::load;

custom_error!{
	pub ConfigError
	Read{path: PathBuf}        = @{format!("{} Failed to read config file", path.display())},
	Write{path: PathBuf}       = @{format!("{} Failed to write config file", path.display())},
	Serialize{path: PathBuf}   = @{format!("{} Failed to serialize config", path.display())},
	Deserialize{path: PathBuf} = @{format!("{} Failed to Deserialize config", path.display())}
}

#[derive(Serialize, Deserialize)]
pub struct Config{
	pub path: PathBuf,
	pub max: i32,
	pub min: i32
}

impl Default for Config{
	fn default() -> Config{
		Config {
			path: PathBuf::from("*"),
			max: 60,
			min: 30
		}
	}
}