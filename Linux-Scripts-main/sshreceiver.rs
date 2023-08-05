use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("Welcome to SSH receiver setup!");
    let mut host = String::new();
    let mut port = String::new();
    let mut user = String::new();

    print!("Enter the remote host IP: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut host).unwrap();
    host = host.trim().to_string();

    print!("Enter the remote host SSH port [22]: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut port).unwrap();
    port = port.trim().to_string();
    if port == "" {
        port = "22".to_string();
    }

    print!("Enter the remote host SSH username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user).unwrap();
    user = user.trim().to_string();

    let ssh_key = "ssh-keygen -t rsa -b 4096 -C 'SSH Receiver' -f ~/.ssh/receiver -N ''";
    let ssh_copy = format!(
        "ssh-copy-id -p {} -i ~/.ssh/receiver.pub {}@{}",
        port, user, host
    );
    let check_ssh = format!(
        "ssh -p {} -i ~/.ssh/receiver {}@{} echo 'SSH receiver setup successful'",
        port, user, host
    );

    Command::new("sh")
        .arg("-c")
        .arg(ssh_key)
        .output()
        .expect("Failed to generate SSH key pair");

    Command::new("sh")
        .arg("-c")
        .arg(ssh_copy)
        .output()
        .expect("Failed to copy SSH public key to remote host");

    let check_ssh_output = Command::new("sh")
        .arg("-c")
        .arg(check_ssh)
        .output()
        .expect("Failed to check SSH receiver setup");

    if check_ssh_output.status.success() {
        println!("SSH receiver setup successful!");
    } else {
        println!("Failed to setup SSH receiver");
    }
}
