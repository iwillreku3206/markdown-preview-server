// Replaces `[pb]` with a page break div.

use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct InlinePageBreak;
impl NodeValue for InlinePageBreak {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("style", "page-break-after:always;break-after:page;".into()));
		attrs.push(("class", "page-break".into()));
        fmt.open("div", &attrs);
		fmt.close("div");
        fmt.cr();
    }
}

struct PageBreakInlineScanner;

impl InlineRule for PageBreakInlineScanner {
    const MARKER: char = '[';
	fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let input = &state.src[state.pos..state.pos_max];

        if !input.starts_with("[pb]") {
            return None;
        }

        Some((Node::new(InlinePageBreak), 5))
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.inline.add_rule::<PageBreakInlineScanner>();
}
