use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path for configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Use stdio
    #[arg(long, default_value_t = false)]
    pub stdio: bool,

	/// Prints loaded configuration
	#[arg(long, default_value_t = false)]
	pub print_config: bool,

    /// Generate default configuration file
    #[arg(long, default_value_t = false)]
    pub generate_default_config: bool,

	#[arg(long)]
	pub compile_file: Option<String>,
}
