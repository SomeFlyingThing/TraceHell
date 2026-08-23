use std::{
    env, io,
    path::{Component, Path, PathBuf},
};

use crate::create::MotherDir;

#[derive(Debug)]
pub struct Target {
    pub path: PathBuf,
    pub command: String,
}
/// create creates the new cpy
/// switch switches to a existing one
/// run runs with the provided commadn
#[derive(Debug)]
pub enum Commands {
    Create(PathBuf),
    Run(Target),
    Delete(PathBuf),
}

pub fn parse() -> io::Result<Commands> {
    let mother_dir = MotherDir::create()?;
    parse_args(env::args().skip(1), &mother_dir)
}

fn parse_args<I, S>(args: I, mother_dir: &Path) -> io::Result<Commands>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);
    let operation = args.next().ok_or_else(|| invalid_input("usage: <-create|-run|-delete> ..."))?;

    match operation.as_str() {
        "-create" => {
            let name = args.next().ok_or_else(|| invalid_input("usage: -create <name>"))?;
            if args.next().is_some() {
                return Err(invalid_input("usage: -create <name>"));
            }

            Ok(Commands::Create(trace_path(mother_dir, &name)?))
        },
        "-run" => {
            let name = args.next().ok_or_else(|| invalid_input("usage: -run <name> <command>"))?;
            let command = args.collect::<Vec<_>>().join(" ");
            if command.is_empty() {
                return Err(invalid_input("usage: -run <name> <command>"));
            }
            let path = trace_path(mother_dir, &name)?;

            if !path.is_dir() {
                return Err(io::Error::new(io::ErrorKind::NotFound, format!("trace '{}' does not exist", name)));
            }
            Ok(Commands::Run(Target { path, command }))
        },
        "-delete" => {
            let name = args.next().ok_or_else(|| invalid_input("usage: -delete <name>"))?;
            if args.next().is_some() {
                return Err(invalid_input("usage: -delete <name>"));
            }
            let path = trace_path(mother_dir, &name)?;

            if !path.is_dir() {
                return Err(io::Error::new(io::ErrorKind::NotFound, format!("trace '{}' does not exist", name)));
            }
            Ok(Commands::Delete(path))
        },
        _ => Err(invalid_input(format!("unknown operation: {operation}"))),
    }
}

fn trace_path(mother_dir: &Path, name: &str) -> io::Result<PathBuf> {
    let mut components = Path::new(name).components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(_)), None) => Ok(mother_dir.join(name)),
        _ => Err(invalid_input("trace name must be a single path component")),
    }
}

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use tempfile::tempdir;

    use super::{Commands, parse_args};

    #[test]
    fn parses_create_path() {
        let mother = tempdir().unwrap();
        let command = parse_args(["-create", "trace"], mother.path()).unwrap();

        match command {
            Commands::Create(path) => assert_eq!(path, mother.path().join("trace")),
            _ => panic!("expected create command"),
        }
    }

    #[test]
    fn parses_multi_word_run_command() {
        let mother = tempdir().unwrap();
        std::fs::create_dir(mother.path().join("trace")).unwrap();
        let command = parse_args(["-run", "trace", "cargo", "run"], mother.path()).unwrap();

        match command {
            Commands::Run(target) => {
                assert_eq!(target.path, mother.path().join("trace"));
                assert_eq!(target.command, "cargo run");
            },
            _ => panic!("expected run command"),
        }
    }

    #[test]
    fn parses_delete_name_instead_of_flag() {
        let mother = tempdir().unwrap();
        std::fs::create_dir(mother.path().join("trace")).unwrap();
        let command = parse_args(["-delete", "trace"], mother.path()).unwrap();

        match command {
            Commands::Delete(path) => assert_eq!(path, mother.path().join("trace")),
            _ => panic!("expected delete command"),
        }
    }

    #[test]
    fn missing_arguments_return_errors() {
        let mother = tempdir().unwrap();

        for args in [vec!["-create"], vec!["-run"], vec!["-run", "trace"], vec!["-delete"]] {
            assert_eq!(parse_args(args, mother.path()).unwrap_err().kind(), ErrorKind::InvalidInput);
        }
    }

    #[test]
    fn rejects_names_outside_the_trace_directory() {
        let mother = tempdir().unwrap();

        for name in ["../trace", "nested/trace", "/tmp/trace", ".", ""] {
            assert_eq!(parse_args(["-create", name], mother.path()).unwrap_err().kind(), ErrorKind::InvalidInput);
        }
    }
}
