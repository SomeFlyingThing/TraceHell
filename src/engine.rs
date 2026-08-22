use std::{
    env,
    fs::{self, File},
    io::{self, Read},
    marker::PhantomData,
    os::unix::fs::MetadataExt,
    path::{self, Component, Components, Path, PathBuf},
    process::Command,
    str::FromStr,
};

use cargo_metadata::MetadataCommand;
use proc_macro2::TokenStream;
use syn::{Expr, parse::Parse, parse_quote, token::Not, visit_mut::VisitMut};
use walkdir::WalkDir;

pub struct NotSaved;

pub trait FileInfoVecExt {
    fn copy_scanfold(&self, future_dir: &Path) -> io::Result<()>;
}
pub struct FileInfo<State> {
    root: PathBuf,
    name: String,
    contents: String,
    path: PathBuf,
    _data: PhantomData<State>,
}

impl FileInfo<NotSaved> {
    pub fn new(path: &Path) -> io::Result<(Vec<Self>, String)> {
        let metadata = MetadataCommand::new().manifest_path(path).exec().unwrap();

        let project_root = metadata.workspace_root.as_std_path().parent().unwrap();

        let mut files = Vec::new();

        for package in metadata.workspace_packages() {
            let path = PathBuf::from(&package.manifest_path);

            let mut file = File::open(&path)?;
            let size = file.metadata()?.size();

            let mut contents = String::with_capacity(size as usize);

            file.read_to_string(&mut contents)?;

            let edited_contents = parse_contents(&mut contents);

            let relative = path.strip_prefix(project_root).unwrap();

            let info = Self {
                root: project_root.to_path_buf(),
                name: relative.file_name().unwrap().to_string_lossy().into_owned(),
                contents: edited_contents,
                path: relative.to_path_buf(),
                _data: PhantomData,
            };

            files.push(info);
        }
        let folder_name = files[0].folder_name();
        Ok((files, folder_name))
    }
    fn folder_name(&self) -> String {
        let folder_name = self
            .path
            .components()
            .next()
            .and_then(|c| match c {
                Component::Normal(name) => Some(name),
                _ => None,
            })
            .unwrap();

        folder_name.to_string_lossy().into()
    }
}

impl FileInfoVecExt for Vec<FileInfo<NotSaved>> {
    fn copy_scanfold(&self, future_dir: &Path) -> io::Result<()> {
        for entry in WalkDir::new(&self[0].root) {
            let entry = entry?;
            let path = entry.path();

            if path.components().any(|c| c.as_os_str() == "target" || c.as_os_str() == ".git") {
                continue;
            }
            let relative = path.strip_prefix(&self[0].root).unwrap();

            if self.iter().any(|file| relative == file.path) {
                continue;
            };

            let dst = future_dir.join(relative);

            if path.is_dir() {
                fs::create_dir_all(&dst)?;
            } else {
                fs::copy(path, &dst)?;
            }
        }
        Ok(())
    }
}

fn parse_contents(contents: &mut str) -> String {
    let tokenstream: proc_macro2::TokenStream = contents.parse().unwrap();

    let mut ast = syn::parse2::<syn::File>(tokenstream).unwrap();

    let mut finder = QuestionMarkFinder;

    finder.visit_file_mut(&mut ast);

    prettyplease::unparse(&ast)
}

struct QuestionMarkFinder;

impl VisitMut for QuestionMarkFinder {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        syn::visit_mut::visit_expr_mut(self, expr);

        let Expr::Try(expr_try) = expr else {
            return;
        };

        let inner = &expr_try.expr;
        *expr = parse_quote! {
        {
            match #inner {
                Ok(__trace_value) => __trace_value,
                Err(__trace_error) => {
                    eprintln!(
                        "{}:{}: {} -> {:?}",
                        file!(),
                        line!(),
                        stringify!(#inner),
                        __trace_error,
                    );
                    return Err(__trace_error.into());
                }
            }
        }
        }
    }
}


