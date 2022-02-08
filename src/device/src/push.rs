use crate::{Device, DeviceError};
use std::fs;

impl Device {
	pub fn push(&self) -> Result<(), DeviceError> {
		{
			let path_pwm_state = self.path.join("pwm1_enable");
			if let Err(_) = fs::write(&path_pwm_state, self.pwm_state.to_string()){
				return Err(DeviceError::WritePwmValue{path: path_pwm_state});
			}
		}
		{
			let path_pwm_value = self.path.join("pwm1");
			if let Err(_) = fs::write(&path_pwm_value, self.pwm_value.to_string()){
				return Err(DeviceError::WritePwmValue {path: path_pwm_value});
			}
		}
		Ok(())
	}
}
