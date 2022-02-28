## build
**dependencs**
- require:
   [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- recommend:
   [git](https://git-scm.com/download/)
```console
git clone https://github.com/Heraclito-Q-Saldanha/amd-gpu-fan-daemon.git
cd amd-gpu-fan-daemon
cargo build --release
```
**optional, strip the binary**
```console
strip target/release/amd-gpu-fan-daemon
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


