mod client;
mod config;
mod server;

use config::{App, Run};
use clap::Parser;
use client::handle_client;
use server::handle_server;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args = App::parse();

    match args.run {
        Run::Server(cmd) => handle_server(cmd),
        Run::Client(cmd) => handle_client(cmd),
    }
}
