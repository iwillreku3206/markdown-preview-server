pub mod parser;
pub mod web;

use crate::parser::parser::parse_file;
use web::web_start;

fn main() -> Result<(), String> {
    let opts = katex::Opts::builder().display_mode(true).build().unwrap();

    //println!("{}", html);
    parse_file(include_str!("../test.md"), Vec::new());
    Ok(())
    //web_start();
}
