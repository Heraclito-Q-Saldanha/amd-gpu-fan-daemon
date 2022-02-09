use crate::{Device, DeviceError};
use std::fs;

impl Device {
	pub fn push(&self) -> Result<(), DeviceError> {
		{
			let path_pwm_state = self.path.join("pwm1_enable");
			if fs::write(&path_pwm_state, self.pwm_state.to_string()).is_err(){
				return Err(DeviceError::WritePwmValue{path: path_pwm_state});
			}
		}
		{
			let path_pwm_value = self.path.join("pwm1");
			if fs::write(&path_pwm_value, self.pwm_value.to_string()).is_err(){
				return Err(DeviceError::WritePwmValue {path: path_pwm_value});
			}
		}
		Ok(())
	}
}
