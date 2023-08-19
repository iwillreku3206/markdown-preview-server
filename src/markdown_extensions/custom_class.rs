use markdown_it::parser::block::{BlockRule, BlockState};
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
enum CustomClassBlockType {
    Open,
    Close,
}

#[derive(Debug)]
pub struct BlockCustomClass {
    class: String,
    block_type: CustomClassBlockType,
}

impl NodeValue for BlockCustomClass {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("class", self.class.clone()));
        if matches!(&self.block_type, CustomClassBlockType::Open) {
            fmt.open("div", &attrs);
        } else {
            fmt.close("div");
        }

        fmt.cr();
    }
}

struct CustomClassBlockScanner;

impl BlockRule for CustomClassBlockScanner {
    fn run(state: &mut BlockState) -> Option<(Node, usize)> {
        let line = state.get_line(state.line).trim();
        if !(line.starts_with("{{") || line.starts_with("}}")) {
            return None;
        }

        if line.starts_with("{{") && line.len() > 2 {
            let class = line[2..].trim();
            if class.is_empty() {
                return None;
            }

            return Some((
                Node::new(BlockCustomClass {
                    class: class.into(),
                    block_type: CustomClassBlockType::Open,
                }),
                1,
            ));
        }

        if line.trim() == "}}" {
            return Some((
                Node::new(BlockCustomClass {
                    class: "".into(),
                    block_type: CustomClassBlockType::Close,
                }),
                1,
            ));
        }

        return None;
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<CustomClassBlockScanner>();
}
