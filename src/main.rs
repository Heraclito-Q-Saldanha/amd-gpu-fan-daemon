use std::path::PathBuf;
use std::{process, thread, time};
use std::env;

fn help(){
	println!(
		"{}\n{}\n{}\n{}\n{}",
		"options:",
		"	-dl  --device-list            | devices list",
		"	-ds  --device-status <device> | device status",
		"	-dsl --device-status-list     | device status list",
		"	-rd  --run-daemon             | run daemon"
	);
	process::exit(1);
}

fn device_list(){
	for dev in device::find(){
		println!("{}", dev.display());
	}
	process::exit(1);
}

fn device_status(path: PathBuf){
	match device::Device::new(path).update(){
		Ok(dev) => println!("{}", dev),
		Err(err) => println!("{}", err)
	}
}

fn device_status_list(){
	for path in device::find(){
		match device::Device::new(path).update(){
			Ok(dev) => println!("{}", dev),
			Err(err) => println!("{}", err)
		}
	}
}

fn run_daemon(config_path: PathBuf){
	let config = {
		let mut path = config_path.clone();
		path.push("devices_config.json");
		config::load(path).unwrap_or(vec![config::Config::default()])
	};
	loop{
		for config in &config{
			for path_device in device::find(){
				if path_device != config.path && PathBuf::from("*") != config.path{
					continue;
				}
				let mut device = device::Device::new(path_device.clone());
				if let Err(e) = device.update(){
					println!("{}", e);
					continue;
				}
				device.pwm_state = device::PwmState::Manual;
				device.pwm_value = {
					let value = (((device.temp_value/1000)-config.min)*device.pwm_max)/(config.max-config.min);
					if value > device.pwm_max{
						device.pwm_max
					}else{
						value
					}
				};
				if let Err(e) = device.push(){
					println!("{}", e);
				}
			}
		}
		thread::sleep(time::Duration::from_secs(5));
	}
}

fn main(){
	let config_path = PathBuf::from("/etc/amd-gpu-fan/");
	config::init(config_path.clone()).expect("fail init config");
	let args: Vec<String> = env::args().collect();
	match args.len() {
		2 => match args[1].as_str(){
			"-dl"  | "--device-list"        => device_list(),
			"-dsl" | "--device-status-list" => device_status_list(),
			"-rd"  | "--run-daemon"         => run_daemon(config_path),
			_ => help()
		},
		3 => match args[1].as_str(){
			"-ds" | "-device-status" => device_status(PathBuf::from(&args[2])),
			_ => help()
		},
		_ => help()
	}
}
