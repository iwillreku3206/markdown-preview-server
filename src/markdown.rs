use std::collections::HashMap;

use markdown_it::{
    plugins::{cmark, extra, html, sourcepos},
    MarkdownIt,
};

use crate::{frontmatter_parser::parser::parse_file_with_frontmatter, markdown_extensions};

pub struct MarkdownParser {
    parser: MarkdownIt,
}

pub enum ParserType {
    Plain,
    Cmark,
    GFM,
    Full,
}

impl MarkdownParser {
    pub fn parse(&self, raw: &str) -> (String, HashMap<String, String>) {
        let file = parse_file_with_frontmatter(raw);

        let ast = self.parser.parse(&file.document_content);
        let render = ast.render();

        (render, file.frontmatter)
    }

    pub fn new(parser_type: ParserType) -> Self {
        match parser_type {
            ParserType::Plain => {
                let mut parser = MarkdownIt::new();
                sourcepos::add(&mut parser);
                return MarkdownParser { parser };
            }
            ParserType::Cmark => {
                let mut parser = MarkdownIt::new();
                cmark::add(&mut parser);
                sourcepos::add(&mut parser);
                return MarkdownParser { parser };
            }
            ParserType::GFM => {
                let mut parser = MarkdownIt::new();
                cmark::add(&mut parser);
                extra::add(&mut parser);
                sourcepos::add(&mut parser);
                return MarkdownParser { parser };
            }
            ParserType::Full => {
                let mut parser = MarkdownIt::new();

                cmark::inline::newline::add(&mut parser);
                cmark::inline::escape::add(&mut parser);
                markdown_extensions::backticks::add(&mut parser);
                cmark::inline::emphasis::add(&mut parser);
                crate::patches::link::add(&mut parser);
                crate::markdown_extensions::image_with_file::add(&mut parser);
                cmark::inline::autolink::add(&mut parser);
                cmark::inline::entity::add(&mut parser);

                cmark::block::fence::add(&mut parser);
                cmark::block::blockquote::add(&mut parser);
                cmark::block::hr::add(&mut parser);
                cmark::block::list::add(&mut parser);
                cmark::block::reference::add(&mut parser);
                cmark::block::lheading::add(&mut parser);
                cmark::block::paragraph::add(&mut parser);
                extra::typographer::add(&mut parser);

                html::add(&mut parser);
                extra::tables::add(&mut parser);
                markdown_extensions::latex::add(&mut parser);
                markdown_extensions::newline::add(&mut parser);
                markdown_extensions::heading_with_id::add(&mut parser);
                markdown_extensions::code_block::add(&mut parser);
                markdown_extensions::equation_graph::add(&mut parser);
                markdown_extensions::custom_class::add(&mut parser);
                markdown_extensions::toc::add(&mut parser);

                markdown_it::plugins::sourcepos::add(&mut parser);

                sourcepos::add(&mut parser);
                return MarkdownParser { parser };
            }
        }
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        MarkdownParser::new(ParserType::Cmark)
    }
}
