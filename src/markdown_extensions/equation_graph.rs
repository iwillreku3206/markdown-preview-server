// Replaces `(\/)-------(\/)` with a nice picture.

use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct BlockEquationGraph {
    equation: String,
}

impl NodeValue for BlockEquationGraph {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs_div = node.attrs.clone();
        attrs_div.push(("class", "equation-graph".into()));


        fmt.cr();
        fmt.open("div", &attrs_div);

        fmt.close("div");
        fmt.cr();
    }
}

struct EquationGraphBlockScanner;

impl BlockRule for EquationGraphBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let line = state.get_line(state.line).trim();
        if !line.starts_with("++") {
            return None;
        }
        if !line.ends_with("++") {
            return None;
        }

        let equation = line.trim_matches('+').trim_matches('+');

        Some((
            Node::new(BlockEquationGraph {
                equation: equation.to_string(),
            }),
            1,
        ))
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<EquationGraphBlockScanner>();
}
