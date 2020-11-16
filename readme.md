# Cloudflare Record Updater
A simple, lightweight and asynchronous application that updates Cloudflare DNS records when your IP address changes.

# Key Features
* Written in Rust
* Fully Asynchronous
* Lightweight
* Configurable
*

## Installation
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


## Configuration
### Cloudflare API Token
Please get a Cloudflare API token from the [Cloudflare dashboard](https://dash.cloudflare.com/profile/api-tokens).
This appliaction requires `Zone.Zone`, `Zone.DNS` with edit permissions
1. Go to the [Cloudflare dashboard API tokens page](https://dash.cloudflare.com/profile/api-tokens)
2. Create Token
3. Create Custom Token
4. Set Token name
5. Add the following in the *Permissions* section
    * `Zone     Zone      Read`
    * `Zone     DNS       Edit`

6. Continue to Summary
7. Make sure you have `All zones - DNS:Edit, Zone:Read`
8. Create token
9. Copy the token


### Editing the Configuration
You can edit the configuration using an terminal editor like `nano`. 
The default location of the config `/etc/cloudflare_updater/Config.Toml`
You can edit it by running this command:
```
sudo nano /etc/cloudflare_updater/Config.Toml
```
The configuration is in the TOML format, which makes it highly readable and easy to understand.


### Configuration Fields
#### `api_key` 
The API KEY for cloudflare to use. 
Paste in the value aqauired from the [instructions](#cloudflare-api-token) above.
Make sure to keep the `'` on either side to make it a valid string.

#### `service`
The IP 'grabber' service to use
Current services are:
* ipify

#### `interval`
The Interval to query the IP and update records in seconds
Can be any number between 1-4294967295.
Either 60, 120 or 300 is recommended for a decent update time.

#### `names`
The records to update via cloudflare
In the format `<record>.<domain>`


#### Example Configs
##### Place Holder
```
api_key = 'ENTER_KEY_HERE'
service = 'ipify'
interval = 300
names = ["server.example.com"]
```
#### Example Config
```
api_key = 'RGJtHeULFELunHmSncnhbjuzGQERcNutGCKTDoaL'
service = 'ipify'
interval = 120
names = ["home.gamersite.tk"]
```


