use clap::Parser;
mod agent;
mod server;

use agent::Agent;
use server::Server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    run: Run,
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Debug, clap::Subcommand)]
enum Run {
    #[command(subcommand)]
    Agent(Agent),
    #[command(subcommand)]
    Server(Server),
}
