mod config;

use clap::Parser;

fn main() {
    let args = config::App::parse();
}
