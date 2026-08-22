use std::{env, io, path::PathBuf};

use crate::create::MotherDir;

pub struct Target {
    pub path: PathBuf,
    pub command: String,
}
/// create creates the new cpy
/// switch switches to a existing one
/// run runs with the provided commadn
pub enum Commands {
    Create(PathBuf),
    Switch(PathBuf),
    Run(Target),
}

pub fn parse() -> io::Result<Commands> {
    let mother_dir = MotherDir::create()?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("empty args");
        return Err(io::Error::new(io::ErrorKind::NotFound, "empty args"));
    }

    match args[1].as_str() {
        "-create" => {
            let path = &args[1].clone();

            let path = PathBuf::from(path);
            if !path.exists() {
                eprintln!("path doesnt exist");
                return Err(io::Error::new(io::ErrorKind::NotFound, "path doestn exist"));
            }

            Ok(Commands::Create(path))
        },
        "-run" => {
            let name = &args[1];
            let path = mother_dir.join(name);
            let command = &args[2];

            //TODO preform a cleaning of command bc of malicius intent
            if !path.exists() {
                eprintln!("path doesnt exist");
                return Err(io::Error::new(io::ErrorKind::NotFound, "path doestn exist"));
            }
            Ok(Commands::Run(Target { path, command: command.clone() }))
        },
        "-switch" => {
            let name = &args[1];
            let path = mother_dir.join(name);
            if !path.exists() {
                eprintln!("path doesnt exist");
                return Err(io::Error::new(io::ErrorKind::NotFound, "path doestn exist"));
            }
            Ok(Commands::Switch(path))
        },
        _ => {
            eprintln!("unkow args");
            Err(io::Error::new(io::ErrorKind::NotFound, "unkown args"))
        },
    }
}
