use anyhow::Result;
use clap::{Args, Parser, Subcommand, ValueEnum};

use service::{create_backend_service, create_frontend_service};

mod service;

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: SubCmd,
}

#[derive(Subcommand)]
enum SubCmd {
    New(NewArgs),
}

#[derive(Args)]
struct NewArgs {
    #[command(subcommand)]
    subcommand: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Back(BackArgs),
    Front(FrontArgs),
}

#[derive(Args)]
pub struct BackArgs {
    path: String,

    #[arg(value_enum, default_value_t = Protocol::GraphQL)]
    protocol: Protocol,

    #[arg(long, default_value_t = false)]
    ssh: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Protocol {
    GraphQL,
    Grpc,
}

#[derive(Args)]
pub struct FrontArgs {
    path: String,

    #[arg(long, default_value_t = false)]
    ssh: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        SubCmd::New(args) => match &args.subcommand {
            Cmd::Back(args) => create_backend_service(&args),
            Cmd::Front(args) => create_frontend_service(&args),
        },
    }
}
