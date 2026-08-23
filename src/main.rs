use std::{ error::Error, fs, os::unix::process::CommandExt, process::Command};

use crate::{
    configs::{Configs, Terminals},
    create::MotherDir,
    engine::{FileInfo, FileInfoVecExt, Save},
    parse::{
        Commands,
        Commands::{Create, Run},
    },
    safety::{check_commad_safety, keep_alive},
};

mod configs;
mod create;
mod engine;
mod head;
mod parse;
mod safety;

fn main() -> std::io::Result<()> {
    let args = parse::parse()?;
    let current_folder_name = MotherDir::create()?;
    let configs = Configs::new()?;

    match args {
        Commands::Create(_path) => {
            let (info, folder_name) = FileInfo::new(&current_folder_name)?;
            //copy the non .rs files
            info.copy_scanfold(&current_folder_name.join(folder_name))?;
            info.iter().for_each(|info| info.save().expect("error"));
        },
        Commands::Run(mut target) => {
            let current_folder_name = current_folder_name.join(target.path);

            //safety checks and add ;bash
            check_commad_safety(&target.command)?;
            keep_alive(&mut target.command);

            Command::new(configs.terminal.to_string())
                .arg("--directory")
                .arg(" ") //space
                .arg(current_folder_name)
                .arg("bash")
                .arg("-lc")
                .arg(target.command)
                .spawn()?;
        },
        Commands::Delete(path) => {
            fs::remove_dir_all(&path)?;
            println!("{} was removed", path.file_name().unwrap().display());
        },
        _ => todo!(),
    }

    Ok(())
}
