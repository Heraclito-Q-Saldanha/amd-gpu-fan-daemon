pub mod init;
pub mod load;
pub mod push;

use serde::{Deserialize, Serialize};
use std::{
	fmt::Display,
	path::PathBuf
};

pub use crate::init::init;
pub use crate::load::load;

#[derive(Debug, Clone)]
pub enum ConfigError{
	Read(PathBuf),
	Write(PathBuf),
	Serialize(PathBuf),
	Deserialize(PathBuf)
}

impl Display for ConfigError{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}",
			match self {
				Self::Serialize(path) => format!("{} Failed to serialize config", path.display()),
				Self::Deserialize(path) => format!("{} Failed to Deserialize config", path.display()),
				Self::Read(path) => format!("{} Failed to read config file", path.display()),
				Self::Write(path) => format!("{} Failed to write config file", path.display())
			}
		)
	}
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