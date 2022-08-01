# ELA-Daemon (Easy Logging Application)

# Description
Daemon for the ELA project. Sends a ping every few minutes to a backend. Designed to work as a service with systemd.

# How to install
1. place the binary file (ela-daemon) under /usr/bin/
2. place the service (ela-service) under /etc/systemd/system/
3. run `sudo systemctl enable ela.service`
4. run `sudo systemctl start ela.service`

# Configuration
## config file
The configuration file is located under `/etc/ela/Settings.toml`
### Possible Settings
- DOMAIN="example.com"
- - the domain (for example: google.com, api.example.com)
- UUID="some uuid"
- - gets created automatically but can be changed (idk why but it's possible :))
## Environment variables
Same possible settings as with the config file.
Need to have the prefix ELA followed by _. (For example: ELA_DOMAIN="example.ch")