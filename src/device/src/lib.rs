pub mod find;
pub mod push;
pub mod update;

pub use crate::find::find;
use custom_error::custom_error;
use std::fmt;
use std::path::PathBuf;

pub enum PwmState{
	None,
	Manual,
	Automatic
}
custom_error! {
	pub DeviceError
	NotExits{path: PathBuf}      = @{format!("{} Device not found", path.display())},
	ReadName{path: PathBuf}      = @{format!("{} Failed to read device name", path.display())},
	ReadPwmValue{path: PathBuf}  = @{format!("{} Failed to read pwm value", path.display())},
	ReadPwmState{path: PathBuf}  = @{format!("{} Failed to read pwm state", path.display())},
	ReadTempValue{path: PathBuf} = @{format!("{} Failed to read temp value", path.display())},
	WritePwmValue{path: PathBuf} = @{format!("{} Failed to write pwm value to device", path.display())},
	WriteState{path: PathBuf}    = @{format!("{} Failed to write pwm state to device", path.display())}
}

pub struct Device {
	pub name: String,
	pub path: PathBuf,
	pub pwm_value: i32,
	pub pwm_max: i32,
	pub pwm_state: PwmState,
	pub temp_value: i32
}

impl Device {
	pub fn new(path: PathBuf) -> Device {
		let mut new_device = Device::default();
		new_device.path = path;
		new_device
	}
	pub fn default() -> Device {
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

impl PwmState{
	pub fn to_string(&self) -> String{
		match self{
			PwmState::None => "0",
			PwmState::Manual => "1",
			PwmState::Automatic => "3",
		}.to_string()
	}
	pub fn from_string(text: String) -> PwmState{
		match text.as_str(){
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
