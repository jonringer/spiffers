#[derive(Debug, clap::Subcommand)]
#[clap(about = "Commands to run or interact with an agent")]
pub enum Agent {
    #[command(subcommand)]
    Api(AgentApi),
    #[clap(about = "Determine agent health status")]
    Healthcheck,
    #[clap(about = "Run the agent")]
    Run,
    #[clap(about = "Validate a SPIFFERS agent configuration file")]
    Validate,
}

#[derive(Debug, clap::Subcommand)]
#[clap(about = "Interact with running agent")]
pub enum AgentApi {
    #[clap(about = "Fetch new X509 SVID from Worload API")]
    Fetch,
    #[clap(about = "Validate JWT through an agent")]
    Validate,
    #[clap(about = "Print updates to Worload API as they occur")]
    Watch,
}
