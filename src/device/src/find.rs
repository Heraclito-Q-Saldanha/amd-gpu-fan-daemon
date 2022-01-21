use std::path::PathBuf;

pub fn find() -> Vec<PathBuf> {
	let mut device_list: Vec<PathBuf> = Vec::new();

	for glob_path in glob::glob("/sys/class/drm/card?/device/hwmon/hwmon?").expect("glob pattern"){
		if let Ok(path) = glob_path {
			device_list.push(path);
		}
	}
	device_list
}