mod client;
mod completion;
mod config;
mod datastore;
mod server;

use clap::{CommandFactory, Parser};
use client::handle_client;
use config::{App, Run};
use server::handle_server;
use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = App::parse();

    match args.run {
        Run::Server(cmd) => handle_server(cmd),
        Run::Client(cmd) => handle_client(cmd),
        Run::Completion { shell } => {
            completion::print_completions(shell, &mut App::command());
            Ok(())
        }
    }
}
