// Replaces `[toc]` with special characters to mark a TOC block for replacement

use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct InlineTOC;
impl NodeValue for InlineTOC {
    fn render(&self, _node: &Node, fmt: &mut dyn Renderer) {
        // replacement character
        fmt.text_raw("�toc�");
        fmt.cr();
    }
}

struct TOCInlineScanner;

impl InlineRule for TOCInlineScanner {
    // This is a character that starts your custom structure
    // (other characters may get skipped over).
    const MARKER: char = '[';

    // This is a custom function that will be invoked on every character
    // in an inline context.
    //
    // It should look for `state.src` exactly at position `state.pos`
    // and report if your custom structure appears there.
    //
    // If custom structure is found, it:
    //  - creates a new `Node` in AST
    //  - returns length of it
    //
    fn run(state: &mut InlineState) -> Option<(Node, usize)> {
        let input = &state.src[state.pos..state.pos_max]; // look for stuff at state.pos

        if !input.starts_with("[toc]") {
            return None;
        } // return None if it's not found

        Some((Node::new(InlineTOC), 5))
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    md.inline.add_rule::<TOCInlineScanner>();
}
