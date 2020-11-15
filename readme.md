# Cloudflare Record Updater
A simple, lightweight and asynchronous application that updates Cloudflare DNS records when your IP address changes.

# Key Features
* Written in Rust
* Fully Asynchronous
* Lightweight
* Configurable
*

# Installation
To build this program from source, you'll need the Rust Toolchain.
It's easily installable from [RustUp](https://rustup.rs/#) for Windows and Unix.
It can be simply installed on Unix by running the following command provided that you have curl installed.
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


### Ubuntu systemwide install (assumes root)
```
cd /tmp
git clone http://git.samhdev.net/samhdev/cloudflare_updater.git
cd cloudflare_updater
cargo build --release
mv target/release/cloudflare_updater /usr/bin/cloudflare_updater
mkdir /etc/cloudflare_updater/
mv install/ExampleConfig.Toml /etc/cloudflare_updater/Config.Toml
mv install/cloudflare_updater.service /sys/systemd/system/cloudflare_updater.service
sudo systemctl daemon-reload
sudo systemctl enable cloudflare_updater

## CONFIGURE THE APPLIACTION
## sudo nano /etc/cloudflare_updater/Config.Toml

sudo systemctl start cloudflare_updater
```



