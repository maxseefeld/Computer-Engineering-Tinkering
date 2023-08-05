#!/bin/bash

# Update the system
echo "Updating the system..."
sudo swupd update

# Install required bundles
echo "Installing required bundles..."
sudo swupd bundle-add clr-security-tools git desktop-apps-extras wireshark nmap gnome-boxes devpkg-openssl devpkg-libpcap devpkg-python3

# Install Zeek
echo "Installing Zeek..."
git clone --recursive https://github.com/zeek/zeek.git
cd zeek
./configure
make
sudo make install
cd ..
export PATH=$PATH:/usr/local/zeek/bin

# Install Snort
echo "Installing Snort..."
wget https://www.snort.org/downloads/snort/daq-2.0.7.tar.gz
wget https://www.snort.org/downloads/snort/snort-2.9.17.tar.gz
tar -xvzf daq-2.0.7.tar.gz
tar -xvzf snort-2.9.17.tar.gz
cd daq-2.0.7
./configure
make
sudo make install
cd ../snort-2.9.17
./configure --enable-sourcefire
make
sudo make install
cd ..

# Install OSSEC
echo "Installing OSSEC..."
wget https://github.com/ossec/ossec-hids/archive/3.6.0.tar.gz
tar -xvzf 3.6.0.tar.gz
cd ossec-hids-3.6.0
sudo ./install.sh
cd ..

# Configure Zeek, Snort, and OSSEC to store logs in /var/log folder
echo "Configuring log storage for Zeek, Snort, and OSSEC..."
sudo mkdir -p /var/log/zeek /var/log/snort /var/log/ossec
sudo chown -R zeek:zeek /var/log/zeek
sudo sed -i 's|LogDir = /usr/local/zeek/logs|LogDir = /var/log/zeek|' /usr/local/zeek/etc/zeekctl.cfg
sudo sed -i 's|var LOG_DIR /usr/local/zeek/logs|var LOG_DIR /var/log/zeek|' /usr/local/zeek/share/zeek/site/local.zeek
sudo sed -i 's|^logdir=.*|logdir=/var/log/ossec|' /var/ossec/etc/ossec.conf
sudo sed -i 's|log_directory /var/log/snort|log_directory /var/log|' /etc/snort/snort.conf

# Restart services to apply new log storage settings
echo "Restarting services to apply new log storage settings..."
sudo systemctl restart zeek
sudo systemctl restart snort
sudo systemctl restart ossec

echo "Zeek, Snort, and OSSEC installation and log configuration completed."
