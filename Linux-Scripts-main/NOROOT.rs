use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

fn disable_root_login() -> std::io::Result<()> {
    let sshd_config_path = Path::new("/etc/ssh/sshd_config");
    let file = File::open(sshd_config_path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    for line in &mut lines {
        if line.starts_with("#PermitRootLogin") {
            *line = "PermitRootLogin no".to_string();
        }
    }

    let file = File::create(sshd_config_path)?;
    let mut writer = BufWriter::new(file);

    for line in &lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}
