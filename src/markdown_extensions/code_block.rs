//! Syntax highlighting for code blocks

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;

use markdown_it::parser::core::CoreRule;
use markdown_it::parser::extset::MarkdownItExt;
use markdown_it::plugins::cmark::block::code::CodeBlock;
use markdown_it::plugins::cmark::block::fence::CodeFence;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct SyntectSnippet {
    pub html: String,
}

impl NodeValue for SyntectSnippet {
    fn render(&self, _: &Node, fmt: &mut dyn Renderer) {
        fmt.open("pre", &[]);
        fmt.open("code", &[]);
        fmt.text_raw(&self.html);
        fmt.close("code");
        fmt.close("pre");
    }
}

#[derive(Debug, Clone, Copy)]
struct SyntectSettings(&'static str);
impl MarkdownItExt for SyntectSettings {}

impl Default for SyntectSettings {
    fn default() -> Self {
        Self("InspiredGitHub")
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.add_rule::<SyntectRule>();
}

pub fn set_theme(md: &mut MarkdownIt, theme: &'static str) {
    md.ext.insert(SyntectSettings(theme));
}

pub struct SyntectRule;
impl CoreRule for SyntectRule {
    fn run(root: &mut Node, _md: &MarkdownIt) {
        let ss = SyntaxSet::load_defaults_newlines();

        root.walk_mut(|node, _| {
            let mut content = None;
            let mut language = None;

            if let Some(data) = node.cast::<CodeBlock>() {
                content = Some(&data.content);
            } else if let Some(data) = node.cast::<CodeFence>() {
                language = Some(data.info.clone());
                content = Some(&data.content);
            }

            if let Some(content) = content {
                let mut syntax = None;
                if let Some(language) = language {
                    syntax = ss.find_syntax_by_token(&language);
                }
                let syntax = syntax.unwrap_or_else(|| ss.find_syntax_plain_text());

                let mut generator = ClassedHTMLGenerator::new_with_class_style(
                    syntax,
                    &ss,
                    ClassStyle::SpacedPrefixed { prefix: "syntect-" },
                );

                for line in content.lines() {
                    let mut _line = line.to_string();
                    _line.push('\n');
                    generator
                        .parse_html_for_line_which_includes_newline(&_line)
                        .unwrap_or_default();
                }

                let output = generator.finalize();

                node.replace(SyntectSnippet { html: output });
            }
        });
    }
}
