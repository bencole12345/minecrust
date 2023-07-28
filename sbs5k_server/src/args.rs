use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(long)]
    /// The endpoint on which this server will listen for connections
    pub endpoint: String,
}
