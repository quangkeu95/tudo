use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    /// RPC url
    #[arg(short, long, env, help_heading = "General")]
    pub rpc_url: String,
}
