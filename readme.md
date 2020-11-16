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

Run theese commands one by one as root. A short description of what theese commands will do:
* Clone the source code
* Build the binary
* Move the binary, example config and service config to the correct location
* Reload, enable and start the service
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

rm -r /tmp/cloudflare_updater
```


# Configuration
Please get a Cloudflare API token from the [Cloudflare dashboard](https://dash.cloudflare.com/profile/api-tokens).
This appliaction requires `Zone.Zone`, `Zone.DNS` with edit permissions
1. Go to the [Cloudflare dashboard API tokens page](https://dash.cloudflare.com/profile/api-tokens)
2. Create Token
3. Create Custom Token
4. Set Token name
5. Add the following in the *Permissions* section
5. 1. `Zone`     `Zone`      `Read`
5. 2. `Zone`     `DNS`      `Edit`


