pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown_extensions;
pub mod util;
pub mod web;

use crate::frontmatter_parser::parser::parse_file_with_frontmatter;
use web::web_start;

fn main() -> Result<(), String> {
    //println!("{}", html);
    let mut file = parse_file_with_frontmatter(include_str!("../test.md"));
    /*println!(
        "m:{}",
        crate::parser::blocks::spec::thematic_break::thematic_break()
            .start
            .is_match("---")
    );*/
    let parser = &mut markdown_it::MarkdownIt::new();
    markdown_it::plugins::cmark::add(parser);
    markdown_it::plugins::html::add(parser);
    markdown_it::plugins::extra::tables::add(parser);
    markdown_it::plugins::extra::syntect::add(parser);
    crate::markdown_extensions::latex::add(parser);
    crate::markdown_extensions::newline::add(parser);
    file.document_content = crate::hooks::toc::toc(file.document_content);

    let ast = parser.parse(&file.document_content);
    let output = ast.render();
    println!("{}", output);

    Ok(())
    //web_start();
}
