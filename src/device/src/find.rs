use std::path::PathBuf;

pub fn find() -> Vec<PathBuf> {
	glob::glob("/sys/class/drm/card?/device/hwmon/hwmon?").expect("glob pattern").filter_map(|p| p.ok()).collect::<Vec<PathBuf>>()
}