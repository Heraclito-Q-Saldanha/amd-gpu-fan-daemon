use crate::{Device, DeviceError};
use std::fs;

impl Device {
	pub fn push(&self) -> Result<(), DeviceError> {
		{
			let mut path_pwm_state = self.path.clone();
			path_pwm_state.push("pwm1_enable");
			if let Err(_) = fs::write(&path_pwm_state, self.pwm_state.to_string()){
				return Err(DeviceError::WritePwmValue{path: path_pwm_state});
			}
		}
		{
			let mut path_pwm_value = self.path.clone();
			path_pwm_value.push("pwm1");
			if let Err(_) = fs::write(&path_pwm_value, self.pwm_value.to_string()){
				return Err(DeviceError::WritePwmValue {path: path_pwm_value});
			}
		}
		Ok(())
	}
}
