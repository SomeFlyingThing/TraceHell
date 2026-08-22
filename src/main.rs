use std::error::Error;

use crate::{create::create, parse::Commands::Create};
mod create;
mod engine;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse::parse()?;
    let head: Option<&str> = None;

    match args {
        Create(path) => {
            let current_folder_name = create()?;
            let head = &current_folder_name;
        },
        _ => todo!()
    }

    Ok(())
}
