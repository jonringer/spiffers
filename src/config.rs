use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct App {

    #[command(subcommand)]
    run: Run
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
enum Run {
  #[command(subcommand)]
  Client(Client),
  #[command(subcommand)]
  Server(Server),
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
enum Client {
    Api,
    Healthcheck,
    Run,
    Validate
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
enum Server {
  #[clap(about = "Manage registered agents")]
  Agent,
  #[clap(about = "Manage CA bundle data")]
  Bundle,
  #[clap(about = "Manage registration entries")]
  Entry,
  #[clap(about = "Determine health of running server")]
  Healthcheck,
  #[command(subcommand)]
  Generate(ServerGenerate),
  #[clap(about = "Run the server")]
  Run,
  #[clap(about = "Validate the server configuration")]
  Validate
}

#[derive(Debug)]
#[derive(clap::Subcommand)]
#[clap(about = "Generate a JWT-SVID, join token, or x509-SVID")]
enum ServerGenerate {
  #[clap(about = "Generate a JWT-SVID")]
  Jwt,
  #[clap(about = "Generate a join token")]
  Token,
  #[clap(about = "Generate a x509-SVID")]
  X509,
}
