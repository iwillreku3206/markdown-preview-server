// Replaces $$ {LaTeX} $$ with HTML.

use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
// This is a structure that represents your custom Node in AST.
pub struct InlineLaTeX {
    pub marker: char,
}

// This defines how your custom node should be rendered.
impl NodeValue for InlineLaTeX {
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

        fmt.close("span");
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into inline subparser
    // md.inline.add_rule::<FerrisInlineScanner>();
    markdown_it::generics::inline::emph_pair::add_with::<'$', 2, true>(md, || {
        Node::new(InlineLaTeX { marker: '$' })
    });
}
