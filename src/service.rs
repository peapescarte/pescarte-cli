use crate::{BackArgs, FrontArgs, Protocol};
use anyhow::Result;
use convert_case::{Case, Casing};
use std::{path::PathBuf, process::Command};
use walkdir::WalkDir;

struct Service {
    path: PathBuf,
    name: String,
    prog: Programming,
    repo: String,
}

enum Programming {
    Backend,
    Frontend,
}

impl Service {
    fn backend(path_str: String, ssh: bool) -> Self {
        let path = PathBuf::from(path_str);
        let repo = get_repo_url(Programming::Backend, ssh);
        let name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Self {
            path,
            repo,
            name,
            prog: Programming::Backend,
        }
    }

    fn frontend(path_str: String, ssh: bool) -> Self {
        let path = PathBuf::from(path_str);
        let repo = get_repo_url(Programming::Frontend, ssh);
        let name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        Self {
            path,
            repo,
            name,
            prog: Programming::Frontend,
        }
    }
}

fn get_repo_url(prog: Programming, ssh: bool) -> String {
    match (prog, ssh) {
        (Programming::Backend, true) => "git@github.com:peapescarte/backend_template".to_string(),
        (Programming::Backend, false) => {
            "https://github.com/peapescarte/backend_template".to_string()
        }
        (Programming::Frontend, true) => "git@github.com:peapescarte/frontend_template".to_string(),
        (Programming::Frontend, false) => {
            "https://github.com/peapescarte/frontend_template".to_string()
        }
    }
}

pub fn create_backend_service(args: &BackArgs) -> Result<()> {
    let service = Service::backend(args.path.clone(), args.ssh);
    clone_service(&service)?;
    apply_protocol_config(&service, args.protocol)?;

    rename_service(&service)
}

pub fn create_frontend_service(args: &FrontArgs) -> Result<()> {
    let service = Service::backend(args.path.clone(), args.ssh);
    clone_service(&service)?;

    rename_service(&service)
}

fn clone_service(service: &Service) -> Result<()> {
    Command::new("git")
        .arg("clone")
        .arg(&service.repo)
        .arg(&service.path)
        .output()?;

    Ok(())
}

fn apply_protocol_config(service: &Service, protocol: Protocol) -> Result<()> {
    let iter = WalkDir::new(&service.path)
        .into_iter()
        .filter_map(|e| e.ok());

    match protocol {
        Protocol::GraphQL => true,
        Protocol::Grpc => false,
    };

    Ok(())
}

fn rename_service(service: &Service) -> Result<()> {
    match service.prog {
        Programming::Backend => rename_elixir_project(&service.path, &service.name)?,
        Programming::Frontend => rename_vue_project(&service.path, &service.name)?,
    };

    Ok(())
}

fn rename_elixir_project(path: &PathBuf, name: &String) -> Result<()> {
    let upper_camel_name = name.to_case(Case::UpperCamel);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {}

    Ok(())
}

fn rename_vue_project(path: &PathBuf, name: &String) -> Result<()> {
    let camel_name = name.to_case(Case::Camel);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {}

    Ok(())
}
