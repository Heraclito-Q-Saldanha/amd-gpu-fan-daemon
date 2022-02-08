use crate::{Device, DeviceError, PwmState};
use std::fs;

impl Device{
	pub fn update(&mut self) -> Result<&Device, DeviceError> {
		if !self.path.exists(){
			return Err(DeviceError::NotExits {path: self.path.clone()});
		}
		self.name = {
			let path_name = self.path.join("name");
			match fs::read_to_string(&path_name) {
				Ok(name) => name.trim_matches('\n').to_string(),
				Err(_) => return Err(DeviceError::ReadName{path: path_name})
			}
		};
		self.pwm_value = {
			let path_pwm_value = self.path.join("pwm1");
			match fs::read_to_string(&path_pwm_value){
				Ok(string_pwm_value) => match string_pwm_value.trim_matches('\n').parse::<i32>(){
					Ok(value) => value,
					Err(_) => return Err(DeviceError::ReadPwmValue{path: path_pwm_value})
				},
				Err(_) => return Err(DeviceError::ReadPwmValue{path: path_pwm_value})
			}
		};
		self.pwm_max = fs::read_to_string(self.path.join("pwm1_max")).unwrap_or(255.to_string()).trim_matches('\n').parse::<i32>().unwrap_or(255);
		self.pwm_state = {
			let path_pwm_state = self.path.join("pwm1_enable");
			match fs::read_to_string(&path_pwm_state){
				Ok(string_pwm_state) => PwmState::from_string(string_pwm_state.trim_matches('\n').to_string()),
				Err(_) => return Err(DeviceError::ReadPwmState{path: path_pwm_state})
			}
		};
		self.temp_value = {
			let path_temp_value = self.path.join("temp1_input");
			match fs::read_to_string(&path_temp_value){
				Ok(string_temp_value) => {
					match string_temp_value.trim_matches('\n').parse::<i32>(){
						Ok(value) => value,
						Err(_) => return Err(DeviceError::ReadTempValue{path: path_temp_value})
					}
				}
				Err(_) => return Err(DeviceError::ReadTempValue{path: path_temp_value})
			}
		};
		Ok(self)
	}
}