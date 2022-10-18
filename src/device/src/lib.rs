pub mod find;
pub mod push;
pub mod update;

pub use crate::find::find;
use std::{
	path::PathBuf,
	fmt::{
		self,
		Display
	}
};

pub enum PwmState{
	None,
	Manual,
	Automatic
}


#[derive(Debug, Clone)]
pub enum DeviceError{
	NotExits(PathBuf),
	ReadName(PathBuf),
	ReadPwmValue(PathBuf),
	ReadPwmState(PathBuf),
	ReadTempValue(PathBuf),
	WritePwmValue(PathBuf),
	WriteState(PathBuf)
}

impl Display for DeviceError{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self{
			Self::NotExits(path) => format!("{} Device not found", path.display()),
			Self::ReadName(path) => format!("{} Failed to read device name", path.display()),
			Self::ReadPwmValue(path) => format!("{} Failed to read pwm value", path.display()),
			Self::ReadPwmState(path) => format!("{} Failed to read pwm state", path.display()),
			Self::ReadTempValue(path) => format!("{} Failed to read temp value", path.display()),
			Self::WritePwmValue(path) => format!("{} Failed to write pwm value to device", path.display()),
			Self::WriteState(path) => format!("{} Failed to write pwm state to device", path.display())
		})
	}
}

pub struct Device {
	pub name: String,
	pub path: PathBuf,
	pub pwm_value: i32,
	pub pwm_max: i32,
	pub pwm_state: PwmState,
	pub temp_value: i32
}

impl Default for Device{
	fn default() -> Device{
		Device {
			name: String::new(),
			path: PathBuf::new(),
			pwm_value: 0,
			pwm_max: 255,
			pwm_state: PwmState::Automatic,
			temp_value: 0
		}
	}
}

impl Device {
	pub fn new(path: PathBuf) -> Device {
		Device{path: path, ..Default::default()}
	}
}

impl Display for PwmState{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str(
			match self{
				PwmState::None => "0",
				PwmState::Manual => "1",
				PwmState::Automatic => "3",
			}
		)
	}
}

impl From<&str> for PwmState{
	fn from(text: &str) -> Self {
		match text{
			"0" => PwmState::None,
			"1" => PwmState::Manual,
			_ => PwmState::Automatic
		}
	}
}

impl fmt::Debug for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "name: {}\npath: {:?}\npwm_value: {}\npwm_max: {}\ntemp: {}", self.name, self.path, self.pwm_value, self.pwm_max, self.temp_value)
	}
}
impl fmt::Display for Device {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "name: {}\npath: {:?}\nfan:  {}%\ntemp: {}°C", self.name, self.path, (self.pwm_value * 100) / self.pwm_max, self.temp_value / 1000)
	}
}
