
# amd-gpu-fan-daemon

a simple daemon to control the fan curvature of amd video cards written in rust

## build
```console
git clone https://github.com/Heraclito-Q-Saldanha/amd-gpu-fan-daemon.git
cd amd-gpu-fan-daemon
cargo build --release
```
## install
```console
sudo cp target/release/amd-gpu-fan-daemon /bin
sudo cp amd-gpu-fan.service /etc/systemd/system
sudo systemctl enable amd-gpu-fan.service
sudo systemctl start amd-gpu-fan.service
```

## unistall
```console
sudo systemctl stop amd-gpu-fan.service
sudo systemctl disable amd-gpu-fan.service
sudo rm -f /etc/systemd/system/amd-gpu-fan.service
sudo rm -f /bin/amd-gpu-fan-daemon
```
