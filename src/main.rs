use std::{error::Error, os::unix::process::CommandExt, process::Command};

use crate::{
    create::MotherDir,
    engine::{FileInfo, FileInfoVecExt, Save},
    parse::Commands::{Create, Run},
    safety::check_commad_safety,
};

mod configs;
mod create;
mod engine;
mod head;
mod parse;
mod safety;

fn main() -> std::io::Result<()> {
    let args = parse::parse()?;
    let _head: Option<&str> = None;
    let current_folder_name = MotherDir::create()?;

    match args {
        Create(_path) => {
            let (info, folder_name) = FileInfo::new(&current_folder_name)?;
            //copy the non .rs files
            info.copy_scanfold(&current_folder_name.join(folder_name))?;
            info.iter().for_each(|info| info.save().expect("error"));
        },
        Run(target) => {
            let current_folder_name = current_folder_name.join(target.path);

            check_commad_safety(&target.command)?;
            
            let command = Command::new(&target.command).current_dir(current_folder_name).exec();

            
        },
        _ => todo!(),
    }

    Ok(())
}
