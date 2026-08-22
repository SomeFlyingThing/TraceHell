use std::error::Error;

use crate::{create::create, parse::Commands::Create};
mod create;
mod engine;
mod parse;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse::parse()?;
    let _head: Option<&str> = None;

    match args {
        Create(_path) => {
            let current_folder_name = create()?;
            let _head = &current_folder_name;
        },
        _ => todo!()
    }

    Ok(())
}
