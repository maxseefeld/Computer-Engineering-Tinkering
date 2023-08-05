#!/bin/bash

# Partition the disk
parted /dev/sda mklabel gpt
parted /dev/sda mkpart primary ext4 1MiB 512MiB
parted /dev/sda set 1 boot on
parted /dev/sda mkpart primary ext4 512MiB 100%
parted /dev/sda print

# Format the partitions
mkfs.ext4 /dev/sda1
cryptsetup luksFormat /dev/sda2
cryptsetup open /dev/sda2 cryptroot
mkfs.ext4 /dev/mapper/cryptroot

# Mount the partitions
mount /dev/mapper/cryptroot /mnt/gentoo
mkdir /mnt/gentoo/boot
mount /dev/sda1 /mnt/gentoo/boot

# Install Gentoo
cd /mnt/gentoo
wget http://distfiles.gentoo.org/releases/amd64/autobuilds/latest-stage3-amd64.txt
latest_stage3=$(cat latest-stage3-amd64.txt | grep -v "^#" | cut -d" " -f1)
wget http://distfiles.gentoo.org/releases/amd64/autobuilds/$latest_stage3
tar xpf $latest_stage3 --xattrs-include='*.*' --numeric-owner
rm $latest_stage3 latest-stage3-amd64.txt
echo 'MAKEOPTS="-j$(nproc)"' >> /mnt/gentoo/etc/portage/make.conf

# Set up Gentoo configuration
cp /etc/resolv.conf /mnt/gentoo/etc/
mount --types proc /proc /mnt/gentoo/proc
mount --rbind /sys /mnt/gentoo/sys
mount --make-rslave /mnt/gentoo/sys
mount --rbind /dev /mnt/gentoo/dev
mount --make-rslave /mnt/gentoo/dev
chroot /mnt/gentoo /bin/bash
source /etc/profile
export PS1="(chroot) ${PS1}"

# Configure the system
emerge-webrsync
emerge --sync
eselect profile set 1
echo "America/New_York" > /etc/timezone
emerge --config sys-libs/timezone-data
echo "en_US ISO-8859-1" > /etc/locale.gen
echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen
locale-gen
eselect locale set 5
env-update && source /etc/profile

# Install and configure the GNOME desktop environment
emerge xorg-server xorg-drivers xterm
emerge gnome-base/gnome gnome-extra/gnome-tweaks gnome-extra/gnome-shell-extensions gnome-extra/gnome-shell-extensions-topicons-plus
