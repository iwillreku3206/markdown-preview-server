use crate::{frontmatter_parser::parser::parse_file_with_frontmatter, markdown_extensions};

pub fn parse_markdown(raw: &str) -> String {
    let file = parse_file_with_frontmatter(raw);

    let parser = &mut markdown_it::MarkdownIt::new();

    markdown_it::plugins::cmark::inline::newline::add(parser);
    markdown_it::plugins::cmark::inline::escape::add(parser);
    crate::markdown_extensions::backticks::add(parser);
    markdown_it::plugins::cmark::inline::emphasis::add(parser);
    crate::patches::link::add(parser);
    crate::markdown_extensions::image_with_file::add(parser);
    markdown_it::plugins::cmark::inline::autolink::add(parser);
    markdown_it::plugins::cmark::inline::entity::add(parser);

    markdown_it::plugins::cmark::block::fence::add(parser);
    markdown_it::plugins::cmark::block::blockquote::add(parser);
    markdown_it::plugins::cmark::block::hr::add(parser);
    markdown_it::plugins::cmark::block::list::add(parser);
    markdown_it::plugins::cmark::block::reference::add(parser);
    markdown_it::plugins::cmark::block::lheading::add(parser);
    markdown_it::plugins::cmark::block::paragraph::add(parser);
    markdown_it::plugins::extra::typographer::add(parser);

    markdown_it::plugins::html::add(parser);
    markdown_it::plugins::extra::tables::add(parser);
    crate::markdown_extensions::latex::add(parser);
    crate::markdown_extensions::newline::add(parser);
    crate::markdown_extensions::heading_with_id::add(parser);
    crate::markdown_extensions::code_block::add(parser);
    crate::markdown_extensions::equation_graph::add(parser);
    crate::markdown_extensions::custom_class::add(parser);
    crate::markdown_extensions::toc::add(parser);

	markdown_it::plugins::sourcepos::add(parser);

    let ast = parser.parse(&file.document_content);
    ast.walk(|node, i| {
        println!(
            "[{i}] {:?} {:?}::{:?}",
            node.srcmap, node.node_type, node.node_value
        );
    });

    let render = ast.render();

    crate::hooks::toc::toc(render, file.document_content)
}
