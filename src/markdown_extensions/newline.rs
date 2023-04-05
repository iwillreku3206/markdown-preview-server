// Replaces \n with newline

use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
// This is a structure that represents your custom Node in AST.
pub struct InlineNewline;
// This defines how your custom node should be rendered.
impl NodeValue for InlineNewline {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        // `node.attrs` are custom attributes added by other plugins
        // (for example, source mapping information)

        fmt.self_close("br", &[]);
        fmt.cr();
    }
}

// This is an extension for the inline subparser.
struct NewlineInlineScanner;

impl InlineRule for NewlineInlineScanner {
    // This is a character that starts your custom structure
    // (other characters may get skipped over).
    const MARKER: char = '\\';

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
        let test_regex = regex::Regex::new(r"\\n|\\$").unwrap();
        if !test_regex.is_match(input) {
            return None;
        } // return None if it's not found

        // return new node and length of this structure
        Some((Node::new(InlineNewline), 6))
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    md.inline.add_rule::<NewlineInlineScanner>();
}
