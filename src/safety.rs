use std::io;

use syn::Error;

pub enum UnsafeCommand {
    Rm,
    DiskWrite,
    PrivilageEscalation,
}

impl From<UnsafeCommand> for std::io::Error {
    fn from(value: UnsafeCommand) -> Self {
        io::Error::new(io::ErrorKind::PermissionDenied, "unsafe operation detected")
    }
}

pub fn check_commad_safety(command: &str) -> Result<(), UnsafeCommand> {
    if command.contains("rm") {
        return Err(UnsafeCommand::Rm);
    }
    if command.contains("dd") || command.contains("wipefs") {
        return Err(UnsafeCommand::DiskWrite);
    }
    if command.contains("sudo") {
        return Err(UnsafeCommand::PrivilageEscalation);
    }

    Ok(())
}
