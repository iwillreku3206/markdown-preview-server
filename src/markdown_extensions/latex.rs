// Replaces $$ {LaTeX} $$ with HTML.

use substring::Substring;

use markdown_it::parser::inline::{InlineRule, InlineState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
// This is a structure that represents your custom Node in AST.
pub struct InlineKaTeX {
    pub marker: char,
}

// This defines how your custom node should be rendered.
impl NodeValue for InlineKaTeX {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        // `node.attrs` are custom attributes added by other plugins
        // (for example, source mapping information)
        let mut attrs = node.attrs.clone();

        attrs.push(("class", "math".into()));

        fmt.open("span", &attrs);

        let mut raw = "".to_owned();

        let _ = &node.children.iter().for_each(|child| {
            let raw_content = child.render();
            raw.push_str(&raw_content);
        });
        
        let result = latex2mathml::latex_to_mathml(&raw, latex2mathml::DisplayStyle::Block)
            .unwrap_or_default();

        fmt.text_raw(&result);

        eprintln!("zzzz{}", raw);
        /*
        eprintln!("{}", self.raw);

        let regex_e = regex::Regex::new(r"\$\$(.+)\$\$").unwrap();
        let latex_orig = regex_e
            .captures(&self.raw)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        let test = regex_e.captures(&self.raw).unwrap();

        eprintln!("aaaa::{}    cccc::{:?}", &self.raw, test);

        let mut latex_from_original = "$$".to_owned();
        latex_from_original.push_str(latex_orig);
        latex_from_original.push_str("$$");
        let output = &self.raw.replace(&latex_from_original, &result);

        // render the LaTeX

        fmt.text_raw(&output);*/
        fmt.close("span");
    }
}

/*
// This is an extension for the inline subparser.
struct FerrisInlineScanner;

impl InlineRule for FerrisInlineScanner {
    // This is a character that starts your custom structure
    // (other characters may get skipped over).
    const MARKER: char = '$';

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
        let test_regex = regex::Regex::new(r"\$\$(.+)\$\$").unwrap();
        if !test_regex.is_match(input) {
            return None;
        } // return None if it's not found
          // return new node and length of this structure
        Some((
            Node::new(InlineKaTeX {
                raw: input.to_string(),
            }),
            input.len(),
        ))
    }
}
*/

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    // md.inline.add_rule::<FerrisInlineScanner>();
    markdown_it::generics::inline::emph_pair::add_with::<'$', 2, true>(md, || {
        Node::new(InlineKaTeX { marker: '$' })
    });
}
