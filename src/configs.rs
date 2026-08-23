use std::{
    env::home_dir,
    fs::OpenOptions,
    io::{self, Read, Write, stdin},
    path::PathBuf,
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
            Terminals::Alacritty => write!(f, "Alacritty"),
            Terminals::Kitty => write!(f, "Kitty"),
            Terminals::GnomeTerminal => write!(f, "GnomeTerminal"),
        }
    }
}
impl Save for Configs {
    fn save(&self) -> std::io::Result<()> {
        let text = toml::to_string_pretty(&self).map_err(|v| io::Error::new(io::ErrorKind::InvalidData, v))?;

        let home = home_dir().unwrap();
        let path = home.join("TraceHall_settings");

        let mut file = OpenOptions::new().create(true).write(true).open(path)?;

        file.write_all(&text.as_bytes())?;
        Ok(())
    }
}

const SETTINGS_FILENAME: &str = "TraceHell";

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
    }
    let mut answer = String::new();
    stdin().read_to_string(&mut answer).expect("unable to read input");
    let settings = loop {
        match answer.to_lowercase().as_str() {
            "a" => break Configs { terminal: Alacritty },
            "b" => break Configs { terminal: Kitty },
            "c" => break Configs { terminal: GnomeTerminal },

            _ => {
                println!("not valid input ");
                continue;
            },
        }
    };
    settings.save().unwrap();

    path
}
impl Configs {
    pub fn new() -> io::Result<Self> {
        let mut file = OpenOptions::new().create(true).truncate(false).read(true).open(get_settings_path())?;

        let mut contetnts = String::new();

        file.read_to_string(&mut contetnts)?;

        let settings = toml::from_str(&contetnts).map_err(|v| io::Error::new(io::ErrorKind::InvalidData, v))?;

        Ok(settings)
    }
}
