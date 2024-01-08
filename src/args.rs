use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path for configuration file
    #[arg(short, long)]
    pub config: Option<String>,
}
