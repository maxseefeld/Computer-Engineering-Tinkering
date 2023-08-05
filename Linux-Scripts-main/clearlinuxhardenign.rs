use std::process::Command;

fn main() {
    // Set system hardening options
    Command::new("swupd")
        .args(&["os-install", "-I", "os-core-hardening", "-v"])
        .output()
        .expect("Failed to install hardening bundle");

    // Enable and configure firewalld
    Command::new("systemctl")
        .args(&["enable", "firewalld"])
        .output()
        .expect("Failed to enable firewalld");

    Command::new("systemctl")
        .args(&["start", "firewalld"])
        .output()
        .expect("Failed to start firewalld");

    Command::new("firewall-cmd")
        .args(&["--set-default-zone", "drop"])
        .output()
        .expect("Failed to set default zone to drop");

    Command::new("firewall-cmd")
        .args(&["--permanent", "--add-service=ssh"])
        .output()
        .expect("Failed to add SSH service to firewalld");

    Command::new("firewall-cmd")
        .args(&["--permanent", "--add-service=http"])
        .output()
        .expect("Failed to add HTTP service to firewalld");

    Command::new("firewall-cmd")
        .args(&["--permanent", "--add-service=https"])
        .output()
        .expect("Failed to add HTTPS service to firewalld");

    Command::new("firewall-cmd")
        .args(&["--reload"])
        .output()
        .expect("Failed to reload firewalld");

    // Set up SELinux
    Command::new("setenforce")
        .arg("1")
        .output()
        .expect("Failed to enable SELinux");

    let selinux_config = "
SELINUX=enforcing
SELINUXTYPE=targeted
";
    std::fs::write("/etc/selinux/config", selinux_config)
        .expect("Failed to configure SELinux");

    Command::new("restorecon")
        .arg("-R")
        .arg("/")
        .output()
        .expect("Failed to restore SELinux contexts");

    // Configure auditd
    let auditd_rules = "
-w /etc/passwd -p wa -k identity
-w /etc/group -p wa -k identity
-w /etc/shadow -p wa -k identity
-w /etc/sudoers -p wa -k sudoers
";
    std::fs::write("/etc/audit/rules.d/audit.rules", auditd_rules)
        .expect("Failed to configure auditd");

    Command::new("systemctl")
        .args(&["enable", "auditd"])
        .output()
        .expect("Failed to enable auditd");

    Command::new("systemctl")
        .args(&["start", "auditd"])
        .output()
        .expect("Failed to start auditd");

    // Restrict access to /tmp directory
    let tmp_config = "
tmpfs /tmp tmpfs defaults,noexec,nosuid,nodev 0 0
";
    std::fs::write("/etc/fstab", tmp_config)
        .expect("Failed to configure /tmp directory");

    Command::new("mount")
        .arg("/tmp")
        .output()
        .expect("Failed to mount /tmp directory");

    // Harden sysctl
    let sysctl_config = "
# IP Spoofing protection
net.ipv4.conf.all.rp_filter = 1
net.ipv4.conf.default.rp_filter = 1

# Ignore ICMP broadcast
