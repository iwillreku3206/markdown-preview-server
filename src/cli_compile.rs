use std::{io::Read, process};

use crate::{args::Args, config::Config, server::Server};

pub fn cli_compile(path: &str, args: &Args, config: Config) {
    let server = Server::new(args, config);

    let mut src = String::new();
    if path == "-" {
        std::io::stdin().read_to_string(&mut src).unwrap();
        server.compiler.parse(&src);
    } else {
        if let Ok(file) = std::fs::read_to_string(path) {
            src = file;
        } else {
            eprintln!("Error: Could not read file {}", path);
            process::exit(1);
        }
    }

    println!("{}", server.compiler.parse(&src));
    process::exit(0);
}
