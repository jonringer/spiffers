use clap;

#[derive(Debug, clap::Subcommand)]
#[clap(about = "Commands to run or interact with an agent")]
pub enum Agent {
    #[command(subcommand)]
    #[clap(about = "Interact with running agent")]
    Api(AgentApi),
    #[clap(about = "Determine agent health status")]
    #[command(subcommand)]
    Healthcheck,
    #[clap(about = "Run the agent")]
    #[command(subcommand)]
    Run,
    #[clap(about = "Validate a SPIFFERS agent configuration file")]
    #[command(subcommand)]
    Validate,
}

#[derive(Debug, clap::Subcommand)]
pub enum AgentApi {
    #[clap(about = "Fetch new X509 SVID from Worload API")]
    Fetch,
    #[command(subcommand)]
    Validate(AgentValidate),
    #[clap(about = "Print updates to Worload API as they occur")]
    Watch,
}

#[derive(Debug, clap::Subcommand)]
#[clap(about = "Validate JWT through an agent")]
pub enum AgentValidate {
    #[clap(about = "Validate JWT through an agent")]
    Jwt(ValidateJwtOptions),
}

#[derive(Debug, clap::Parser)]
pub struct ValidateJwtOptions {
    #[arg(short, long, default_value = "/tmp/spire-agent/public/api.sock", help = "Path to agent socket", env = "SPIFFERS_AGENT_SOCKET_PATH")]
    socket_path: String,
}
