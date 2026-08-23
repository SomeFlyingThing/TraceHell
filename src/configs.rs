use std::{
    env::home_dir,
    fs::{File, OpenOptions},
    io::{self, Read, Write, stdin},
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::{
    Save,
    configs::Terminals::{Alacritty, GnomeTerminal, Kitty},
};

#[derive(Serialize, Deserialize)]
pub struct Configs {
    pub terminal: Terminals,
}

#[derive(Serialize, Deserialize)]
pub enum Terminals {
    Alacritty,
    Kitty,
    GnomeTerminal,
}
use std::fmt;

impl fmt::Display for Terminals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Terminals::Alacritty => write!(f, "alacritty"),
            Terminals::Kitty => write!(f, "kitty"),
            Terminals::GnomeTerminal => write!(f, "gnome-terminal"),
        }
    }
}

impl Terminals {
    pub fn command(&self, directory: &Path, shell_command: &str) -> Command {
        let mut command = Command::new(self.to_string());

        match self {
            Terminals::Alacritty => {
                command.arg("--working-directory").arg(directory).arg("-e");
            },
            Terminals::Kitty => {
                command.arg("--directory").arg(directory);
            },
            Terminals::GnomeTerminal => {
                command.arg(format!("--working-directory={}", directory.display())).arg("--");
            },
        }

        command.arg("bash").arg("-lc").arg(shell_command);
        command
    }
}

impl Save for Configs {
    fn save(&self) -> std::io::Result<()> {
        let text = toml::to_string_pretty(&self).map_err(|v| io::Error::new(io::ErrorKind::InvalidData, v))?;

        let home = home_dir().unwrap();
        let path = home.join("TraceHall_settings");

        let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path)?;

        file.write_all(&text.as_bytes())?;
        Ok(())
    }
}

fn get_settings_path() -> PathBuf {
    let home = home_dir().unwrap();
    let path = home.join("TraceHall_settings");

    if !path.exists() {
        println!("what is the terminal type");
        println!(
            "
        A: Alacritty,\n
        B: Kitty,\n
        C: GnomeTerminal,"
        );
        let mut answer = String::new();
        stdin().read_line(&mut answer).expect("unable to read input");
        let settings = loop {
            match answer.trim().to_lowercase().as_str() {
                "a" => break Configs { terminal: Alacritty },
                "b" => break Configs { terminal: Kitty },
                "c" => break Configs { terminal: GnomeTerminal },

                _ => {
                    println!("not valid input ");
                    answer.clear();
                    stdin().read_line(&mut answer).expect("unable to read input");
                },
            }
        };
        settings.save().unwrap();

        print!("\x1B[2J\x1B[H");
    }

    path
}
impl Configs {
    pub fn new() -> io::Result<Self> {
        let mut file = File::open(get_settings_path())?;

        let mut contetnts = String::new();

        file.read_to_string(&mut contetnts)?;

        let settings = toml::from_str(&contetnts).map_err(|v| io::Error::new(io::ErrorKind::InvalidData, v))?;

        Ok(settings)
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, path::Path};

    use super::Terminals;

    fn assert_command(terminal: Terminals, program: &str, expected_args: &[&str]) {
        let command = terminal.command(Path::new("/tmp/trace"), "cargo run; bash");
        let args: Vec<_> = command.get_args().collect();

        assert_eq!(command.get_program(), OsStr::new(program));
        assert_eq!(args, expected_args.iter().map(OsStr::new).collect::<Vec<_>>());
    }

    #[test]
    fn builds_kitty_command() {
        assert_command(Terminals::Kitty, "kitty", &["--directory", "/tmp/trace", "bash", "-lc", "cargo run; bash"]);
    }

    #[test]
    fn builds_alacritty_command() {
        assert_command(Terminals::Alacritty, "alacritty", &["--working-directory", "/tmp/trace", "-e", "bash", "-lc", "cargo run; bash"]);
    }

    #[test]
    fn builds_gnome_terminal_command() {
        assert_command(Terminals::GnomeTerminal, "gnome-terminal", &["--working-directory=/tmp/trace", "--", "bash", "-lc", "cargo run; bash"]);
    }
}
