#[derive(Debug, clap::Subcommand)]
#[clap(about = "Commands to run or interact with an server")]
pub enum Server {
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
    Validate,
}

#[derive(Debug, clap::Subcommand)]
#[clap(about = "Generate a JWT-SVID, join token, or x509-SVID")]
pub enum ServerGenerate {
    #[clap(about = "Generate a JWT-SVID")]
    Jwt,
    #[clap(about = "Generate a join token")]
    Token,
    #[clap(about = "Generate a x509-SVID")]
    X509,
}
