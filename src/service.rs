use crate::{BackArgs, FrontArgs, Protocol};
use anyhow::{bail, Result};
use convert_case::{Case, Casing};
use std::path::Path;
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
        (Programming::Frontend, true) => "git@github.com:peapescarte/frontend_template".to_string(),
        (Programming::Backend, false) => {
            "https://github.com/peapescarte/backend_template".to_string()
        }
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
    let out = Command::new("git")
        .arg("clone")
        .arg(&service.repo)
        .arg(&service.path)
        .output()?;

    // TODO

    if !out.stderr.is_empty() {
        println!("STDERR: {:?}", String::from_utf8(out.stderr)?);
        return bail!("Failed to clone repository!");
    };

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
        Programming::Frontend => rename_cljs_project(&service.path, &service.name)?,
    };

    Ok(())
}

fn rename_elixir_project(path: &PathBuf, name: &String) -> Result<()> {
    let upper_camel_name = name.to_case(Case::UpperCamel);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let content = std::fs::read_to_string(entry.path())?;

        let rename = content
            .as_str()
            .replace("ServiceTemplate", &upper_camel_name)
            .replace("service_template", name);

        if entry
            .file_name()
            .to_str()
            .unwrap()
            .contains(&"service_template")
        {
            std::fs::remove_file(entry.path())?;

            let path_renamed = entry
                .path()
                .as_os_str()
                .to_str()
                .unwrap()
                .replace("service_template", name);

            let path = Path::new(&path_renamed);

            std::fs::write(path, rename)?;
        } else {
            std::fs::write(entry.path(), rename)?;
        }
    }

    Ok(())
}

fn rename_cljs_project(path: &PathBuf, name: &String) -> Result<()> {
    let camel_name = name.to_case(Case::Camel);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {}

    Ok(())
}
