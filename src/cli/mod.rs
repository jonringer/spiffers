use clap::Parser;
use clap_complete::Shell;
mod agent;
mod server;

pub use agent::Agent;
pub use server::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub commands: Commands,
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Agent(agent::Agent),
    #[command(subcommand)]
    Server(Server),
    #[command(value_parser = value_parser!(Shell))]
    Completion { shell: Shell },
}
