use std::process::Command;

fn main() {
    // Update packages and install necessary tools
    Command::new("yum")
        .arg("update")
        .output()
        .expect("Failed to update packages");

    Command::new("yum")
        .args(&["install", "epel-release"])
        .output()
        .expect("Failed to install EPEL repository");

    Command::new("yum")
        .args(&["install", "-y", "fail2ban", "auditd", "selinux-policy-targeted", "yum-utils"])
        .output()
        .expect("Failed to install necessary tools");

    // Configure fail2ban
    let fail2ban_config = "
[sshd]
enabled = true
maxretry = 3
bantime = 600
";
    std::fs::write("/etc/fail2ban/jail.local", fail2ban_config)
        .expect("Failed to configure fail2ban");

    Command::new("systemctl")
        .args(&["enable", "fail2ban"])
        .output()
        .expect("Failed to enable fail2ban");

    Command::new("systemctl")
        .args(&["start", "fail2ban"])
        .output()
        .expect("Failed to start fail2ban");

    // Configure auditd
    let auditd_rules = "
-w /etc/passwd -p wa -k identity
-w /etc/group -p wa -k identity
-w /etc/shadow -p wa -k identity
-w /etc/sudoers -p wa -k sudoers
";
    std::fs::write("/etc/audit/audit.rules", auditd_rules)
        .expect("Failed to configure auditd");

    Command::new("systemctl")
        .args(&["restart", "auditd"])
        .output()
        .expect("Failed to restart auditd");

    // Configure SELinux
    Command::new("setenforce")
        .arg("1")
        .output()
        .expect("Failed to enable SELinux");

    // Configure yum-utils
    Command::new("yum-config-manager")
        .args(&["--disable", "rhui-REGION-rhel-server-extras"])
        .output()
        .expect("Failed to disable RHEL extras repository");

    Command::new("yum-config-manager")
        .args(&["--disable", "rhui-REGION-rhel-server-optional"])
        .output()
        .expect("Failed to disable RHEL optional repository");

    Command::new("yum-config-manager")
        .args(&["--save", "--setopt=rhel-7-server-rpms.priority=1"])
        .output()
        .expect("Failed to set RHEL RPMs priority");

    // Secure /tmp directory
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

# Ignore ICMP broadcast requests
net.ipv4.icmp_echo_ignore_broadcasts = 1

# Disable source packet routing
net.ipv4.conf.all.accept_source_route = 0
net.ipv6.conf.all.accept_source_route
