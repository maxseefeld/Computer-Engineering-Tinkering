use std::process::Command;

fn main() {
    // Update packages and install necessary tools
    Command::new("apt")
        .arg("update")
        .output()
        .expect("Failed to update packages");

    Command::new("apt")
        .arg("upgrade")
        .output()
        .expect("Failed to upgrade packages");

    Command::new("apt")
        .arg("install")
        .args(&["unattended-upgrades", "fail2ban", "ufw", "auditd", "apparmor"])
        .output()
        .expect("Failed to install necessary tools");

    // Configure unattended upgrades
    Command::new("dpkg-reconfigure")
        .args(&["--priority=low", "unattended-upgrades"])
        .output()
        .expect("Failed to configure unattended upgrades");

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

    // Configure UFW
    Command::new("ufw")
        .args(&["default", "deny", "incoming"])
        .output()
        .expect("Failed to deny incoming traffic");

    Command::new("ufw")
        .args(&["default", "allow", "outgoing"])
        .output()
        .expect("Failed to allow outgoing traffic");

    Command::new("ufw")
        .args(&["allow", "ssh"])
        .output()
        .expect("Failed to allow SSH traffic");

    Command::new("ufw")
        .arg("enable")
        .output()
        .expect("Failed to enable UFW");

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

    // Configure AppArmor
    Command::new("systemctl")
        .args(&["enable", "apparmor"])
        .output()
        .expect("Failed to enable AppArmor");

    Command::new("systemctl")
        .args(&["start", "apparmor"])
        .output()
        .expect("Failed to start AppArmor");
}
