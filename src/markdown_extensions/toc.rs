// Counts the number of crabs lurking around.

use std::cmp::Ordering;

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::parser::core::CoreRule;
use markdown_it::plugins::cmark::block::heading::ATXHeading;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

struct Heading {
    level: u8,
    content: String,
}

#[derive(Debug)]
pub struct TOCBlockWrapper;

impl NodeValue for TOCBlockWrapper {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs_div = node.attrs.clone();
        attrs_div.push(("class", "table-of-contents".into()));

        fmt.open("div", &attrs_div);
        fmt.contents(&node.children);
        fmt.close("div");
    }
}

// This is an extension for the block subparser.
struct TOCBlockWrapperScanner;

impl BlockRule for TOCBlockWrapperScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        // get contents of a line number `state.line` and check it
        let line = state.get_line(state.line).trim();
        if !line.starts_with("[toc]") {
            return None;
        }
        // return new node and number of lines it occupies
        Some((Node::new(TOCBlockWrapper), 1))
    }
}

#[derive(Debug)]
pub struct TOCBlock {
    headers: Vec<(u8, String)>,
}

// This defines how your custom node should be rendered.
impl NodeValue for TOCBlock {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let attrs = node.attrs.clone();
        fmt.open("div", &attrs);

        let mut current_level = 1;

        self.headers.iter().for_each(|(level, content)| {
            match level.cmp(&current_level) {
                Ordering::Greater => {
                    fmt.open("ul", &[]);
                }
                Ordering::Less => {
                    for _ in 0..current_level - level {
                        fmt.close("ul");
                    }
                }
                _ => {}
            }

            fmt.open("li", &[]);
            fmt.open(
                "a",
                &[(
                    "href",
                    "#".to_string() + &content.to_lowercase().replace(' ', "-"),
                )],
            );
            fmt.text(content);
            fmt.close("a");
            fmt.close("li");

            current_level = *level;
        });

        fmt.close("div");
    }
}

pub struct TOCRule;

impl CoreRule for TOCRule {
    fn run(root: &mut Node, _: &MarkdownIt) {
        let mut headings: Vec<(u8, String)> = Vec::new();

        // walk through AST recursively and count the number of headings
        root.walk(|node, _| {
            if node.is::<ATXHeading>() {
                let node_value = node.cast() as Option<&ATXHeading>;
                if let Some(node_value) = node_value {
                    let level = node_value.level;
                    let text = node.collect_text();
                    headings.push((level, text));
                }
            }
        });

        // append a child to the wrapper nodes
        root.walk_mut(|node, _| {
            if node.is::<TOCBlockWrapper>() {
                let toc_node = Node::new(TOCBlock {
                    headers: headings.clone(),
                });
                node.children.push(toc_node);
            }
        });
    }
}
pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<TOCBlockWrapperScanner>();
    md.add_rule::<TOCRule>();
}
