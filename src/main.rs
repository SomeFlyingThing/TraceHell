use std::fs;

use crate::{
    configs::Configs,
    create::MotherDir,
    engine::{FileInfo, FileInfoVecExt, Save},
    parse::Commands,
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

    match args {
        Commands::Help => println!("{}", parse::HELP),
        Commands::Create(path) => {
            let (info, _folder_name) = FileInfo::new(&std::env::current_dir()?)?;
            //copy the non .rs files
            info.copy_scanfold(&path)?;
            info.iter().for_each(|info| info.save_to(&path).expect("error"));
            println!("{} was created", path.file_name().unwrap().display());
        },
        Commands::Run(mut target) => {
            let current_folder_name = current_folder_name.join(target.path);
            let configs = Configs::new()?;

            //safety checks and add ;bash
            check_commad_safety(&target.command)?;
            keep_alive(&mut target.command);

            configs.terminal.command(&current_folder_name, &target.command).spawn()?;
        },
        Commands::Delete(path) => {
            fs::remove_dir_all(&path)?;
            println!("{} was removed", path.file_name().unwrap().display());
        },
    }

    Ok(())
}
