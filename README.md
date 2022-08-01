# ELA-Daemon (Easy Logging Application)

# Description
Daemon for the ELA project. Sends a ping every few minutes to a backend. Designed to work as a service with systemd.

# How to install
1. place the binary file (ela-daemon) under /usr/bin/
2. place the service (ela-service) under /etc/systemd/system/
3. run `sudo systemctl enable ela.service`
4. run `sudo systemctl start ela.service`