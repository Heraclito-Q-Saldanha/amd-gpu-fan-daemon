use crate::{Config, ConfigError};
use std::fs;
use std::path::PathBuf;

pub fn load(path_file: PathBuf) -> Result<Vec<Config>, ConfigError> {
	let string_config = match fs::read_to_string(&path_file) {
		Ok(string) => string,
		Err(_) => return Err(ConfigError::Read{path: path_file})
	};
	let mut config_list: Vec<Config> = match serde_json::from_str(&string_config) {
		Ok(config) => config,
		Err(_) => return Err(ConfigError::Serialize{path: path_file})
	};
	for mut config in &mut config_list{
		if config.max <= config.min{
			config.max = config.min + 1;
		}
	}
	Ok(config_list)
}
