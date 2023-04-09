pub mod frontmatter_parser;
pub mod hooks;
pub mod markdown_extensions;
pub mod util;
pub mod web;
pub mod markdown;

use futures_channel::mpsc::UnboundedSender;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tungstenite::Message;

use crate::frontmatter_parser::parser::parse_file_with_frontmatter;

pub type Tx = UnboundedSender<Message>;
pub type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[tokio::main]
async fn main() {
    env_logger::init();
    //println!("{}", html);
    let mut file = parse_file_with_frontmatter(include_str!("../test2.md"));
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
    // let output = ast.render();
    let output = markdown::parse_markdown(&file.document_content);
    println!("{}", output);

    let sessions = PeerMap::new(Mutex::new(HashMap::new()));

    let _ = tokio::join!(
        tokio::spawn(crate::web::ws::ws_start(sessions.clone())),
        tokio::spawn(crate::web::web_start(sessions.clone()))
    );
}
