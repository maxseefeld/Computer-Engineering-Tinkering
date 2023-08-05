use std::process::Command;

fn main() {
    // Set system hardening options
    Command::new("emerge")
        .args(&["--ask", "--verbose", "apparmor", "sys-kernel/hardened-sources"])
        .output()
        .expect("Failed to install hardening packages");

    // Configure AppArmor
    let apparmor_config = "
/usr/sbin/tcpdump {
  # This profile prevents the execution of any program besides tcpdump
  # with the tcpdump binary.
  # It also prevents the execution of any shell and their fork/exec system
  # calls. This way, the program can't execute a shell and bypass this
  # profile.
  # We deny everything by default and then we allow only what we really
  # need.

  # Include generic rules
  #include <abstractions/base>
  #include <abstractions/nameservice>

  # TCPDump can read from /proc/net, but it needs to read from any net file.
  /proc/net/** r,
  /sys/class/net/** r,

  # TCPDump needs to read the link-level header on ethernet devices.
  /sys/class/net/eth0/device/r*,

  # Allow to read libpcap and its configuration files
  /usr/lib*/libpcap.so* mr,
  /usr/share/tcpdump/** r,

  # TCPDump can output files to any file under /tmp
  /tmp/* rw,
  /tmp/** rw,

  # Allow DNS queries
  /etc/resolv.conf r,

  # Allow to dump traffic
  /dev/** rw,
  /dev/kmsg rw,

  # Deny everything else
  deny /bin/** ix,
  deny /boot/** ix,
  deny /lib/** ix,
  deny /lib64/** ix,
  deny /media/** rw,
  deny /mnt/** rw,
  deny /opt/** ix,
  deny /root/** rw,
  deny /sbin/** ix,
  deny /usr/** ix,
  deny /var/** rw,

  # Deny execution of shells and their fork/exec system calls
  deny /bin/sh ix,
  deny /bin/bash ix,
  deny /bin/dash ix,
  deny /bin/zsh ix,
  deny /bin/csh ix,
  deny /bin/ksh ix,
  deny /bin/tcsh ix,
  deny /bin/sash ix,
  deny /bin/busybox ix,
  deny /usr/bin/perl* ix,
  deny /usr/bin/python* ix,
  deny /usr/bin/ruby* ix,
  deny /usr/bin/php* ix,
  deny /usr/bin/gawk* ix,
  deny /usr/bin/awk* ix,
  deny /usr/bin/mawk* ix,
  deny /usr/bin/mksh* ix,
  deny /usr/bin/ash* ix,
  deny /usr/bin/dash* ix,
  deny /usr/bin/zsh* ix,
  deny /usr/bin/lua* ix,
  deny /usr/bin/node* ix,
  deny /usr/bin/npm* ix,
  deny /usr/bin/electron* ix,
  deny /usr/bin/shell* ix,
  deny /usr/bin/gnome* ix,
  deny /usr/bin/gsettings* ix,
  deny /usr/bin/gconftool* ix,
  deny /usr/bin/dbus* ix,
  deny /usr/bin/dbus-daemon* ix,
  deny /usr/bin/dbus-launch* ix,
  deny /usr/bin/xfce* ix,
