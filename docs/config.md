## config
```console
/etc/amd-gpu-fan/devices_config.json
```
- **path** path to device, using * all devices
- **min** minimum temperature, fan turns off
- **max** Maximum temperature, fan 100%

**example:**
```JSON
[
	{
		"path": "*",
		"max": 60,
		"min": 30
	},
	{
		"path": "/sys/class/drm/card0/device/hwmon/hwmon0",
		"max": 50,
		"min": 25
	},
	{
		"path": "/sys/class/drm/card1/device/hwmon/hwmon0",
		"max": 55,
		"min": 20
	}
]
```
