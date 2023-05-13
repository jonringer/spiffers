mod client;
mod completion;
mod cli;
mod datastore;
mod server;

use clap::{CommandFactory, Parser};
use client::handle_client;
use cli::{App, Commands};
use server::handle_server;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = App::parse();

    match args.commands {
        Commands::Server(cmd) => handle_server(cmd),
        Commands::Agent(cmd) => handle_client(cmd),
        Commands::Completion { shell } => {
            completion::print_completions(shell, &mut App::command());
            Ok(())
        }
    }
}
