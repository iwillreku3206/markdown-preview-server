use std::process;

use crate::{args::Args, config::Config};

pub fn generate_defaults(args: &Args) {
    if args.generate_default_config {
        generate_default_config();
    }
}

fn generate_default_config() {
    let default_cfg = Config::default();
    let default_cfg_str = toml::to_string(&default_cfg).unwrap();
    println!("{}", default_cfg_str);
    process::exit(0);
}
