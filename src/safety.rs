use std::io;

#[derive(Debug)]
pub enum UnsafeCommand {
    Rm,
    DiskWrite,
    PrivilageEscalation,
}

impl From<UnsafeCommand> for std::io::Error {
    fn from(_value: UnsafeCommand) -> Self {
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

pub fn keep_alive(command: &mut String) {
    command.push_str("; bash");
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn t_keep_alive(){
        let mut command = String::from("cargo run");

        let result = String::from("cargo run; bash");
        keep_alive(&mut command);

        assert!(command == result);
    }
}
