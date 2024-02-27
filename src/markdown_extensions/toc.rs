// Counts the number of crabs lurking around.

use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

struct Heading {
    level: u8,
    content: String,
}

#[derive(Debug)]
// This is a structure that represents your custom Node in AST,
// it has one single argument - crab counter.
pub struct FerrisCounter(usize);

#[derive(Debug)]
pub struct TOCBlock;

#[derive(Debug)]
pub struct TOCScanner;

// This defines how your custom node should be rendered.
impl NodeValue for FerrisCounter {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        // `node.attrs` are custom attributes added by other plugins
        // (for example, source mapping information)
        let mut attrs = node.attrs.clone();

        // add a custom class attribute
        attrs.push(("class", "ferris-counter".into()));

        fmt.cr(); // linebreak, multiples get merged
        fmt.open("footer", &attrs);
        #[allow(clippy::useless_format)] // for simplicity's sake
        fmt.text(&match self.0 {
            0 => format!("No crabs around here."),
            1 => format!("There is a crab lurking in this document."),
            _ => format!("There are {} crabs lurking in this document.", self.0),
        });
        fmt.close("footer");
        fmt.cr();
    }
}

// This is an extension for the markdown parser.
struct FerrisCounterRule;

impl CoreRule for FerrisCounterRule {
    // This is a custom function that will be invoked once per document.
    //
    // It has `root` node of the AST as an argument and may modify its
    // contents as you like.
    //
    fn run(root: &mut Node, _: &MarkdownIt) {
        let mut headings: Vec<String> = Vec::new();

        // walk through AST recursively and count the number of two
        // custom nodes added by other two rules
        root.walk(|node, _| {
            if node.is::<ATXHeading>() {
                let node_value = node.cast() as Option<&ATXHeading>;
                if let Some(node_value) = node_value {
                    let level = node_value.level;
                    if let Some(content) = node.children.first() {
                        eprintln!("{:#?}", content.collect_text());
                    }
                }
            }
        });

        // append a counter to the root as a custom node
        root.children.push(Node::new(FerrisCounter(0)))
    }
}

pub fn add(md: &mut MarkdownIt) {
    // insert this rule into parser
    md.add_rule::<FerrisCounterRule>();
}
