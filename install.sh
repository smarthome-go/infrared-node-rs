#!/bin/bash

grep -q BCM /proc/cpuinfo || { echo -e "\x1b[1;31mPlease run the installer on a Raspberry pi\x1b[1;0m"; exit 1; }
echo -e "\x1b[1;34minstallation is running\x1b[1;0m"

sudo mkdir -p /etc/ifrs/
sudo chown -R pi /etc/ifrs/ || exit 1
sudo mv ./infrared-node-rs /usr/bin/ifrs || exit 1
sudo cp ./ifrs.service /lib/systemd/system/ifrs.service || exit 1

# Reload Systemd
sudo systemctl daemon-reload || exit 1
sudo systemctl enable ifrs --now || exit 1

echo -e "\x1b[1;32minstallation completed\x1b[1;0m"
