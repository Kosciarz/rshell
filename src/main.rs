use std::{
    error::Error,
    io::{self, Write},
    process::Command,
    result::Result,
};

fn main() -> Result<(), Box<dyn Error>> {
    print!("rshell> ");
    io::stdout().flush()?;

    let mut command = String::new();
    io::stdin().read_line(&mut command)?;
    let command: &str = command.trim();

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(command)
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
