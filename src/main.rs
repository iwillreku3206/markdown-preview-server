pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown_extensions;
pub mod util;
pub mod web;

use tokio::task::futures;

use crate::frontmatter_parser::parser::parse_file_with_frontmatter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    //println!("{}", html);
    let mut file = parse_file_with_frontmatter(include_str!("../test.md"));
    /*println!(
        "m:{}",
        crate::parser::blocks::spec::thematic_break::thematic_break()
            .start
            .is_match("---")
    );*/
    let parser = &mut markdown_it::MarkdownIt::new();

    markdown_it::plugins::cmark::inline::newline::add(parser);
    markdown_it::plugins::cmark::inline::escape::add(parser);
    markdown_it::plugins::cmark::inline::backticks::add(parser);
    markdown_it::plugins::cmark::inline::emphasis::add(parser);
    markdown_it::plugins::cmark::inline::link::add(parser);
    markdown_it::plugins::cmark::inline::image::add(parser);
    markdown_it::plugins::cmark::inline::autolink::add(parser);
    markdown_it::plugins::cmark::inline::entity::add(parser);

    markdown_it::plugins::cmark::block::code::add(parser);
    markdown_it::plugins::cmark::block::fence::add(parser);
    markdown_it::plugins::cmark::block::blockquote::add(parser);
    markdown_it::plugins::cmark::block::hr::add(parser);
    markdown_it::plugins::cmark::block::list::add(parser);
    markdown_it::plugins::cmark::block::reference::add(parser);
    markdown_it::plugins::cmark::block::lheading::add(parser);
    markdown_it::plugins::cmark::block::paragraph::add(parser);

    markdown_it::plugins::html::add(parser);
    markdown_it::plugins::extra::tables::add(parser);
    markdown_it::plugins::extra::syntect::add(parser);
    crate::markdown_extensions::latex::add(parser);
    crate::markdown_extensions::newline::add(parser);
    crate::markdown_extensions::heading_with_id::add(parser);
    file.document_content = crate::hooks::toc::toc(file.document_content);

    let ast = parser.parse(&file.document_content);
    let _output = ast.render();
    //println!("{}", output);
    loop {
        tokio::spawn(async move { crate::web::ws::ws_start() });
        tokio::spawn(async move { crate::web::web_start() });
    }
}
