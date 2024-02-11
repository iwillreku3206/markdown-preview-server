use std::{collections::HashMap, sync::Mutex};

use katex::Opts;
use lazy_static::lazy_static;
use markdown_it::{parser::block::BlockRule, MarkdownIt, Node, NodeValue};

lazy_static! {
    pub static ref KATEX_CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref KATEX_OPTS: Opts = Opts::builder()
        .throw_on_error(false)
        .display_mode(true)
        .build()
        .unwrap();
}

pub fn render_latex(latex: &str) -> String {
    let mut cache = KATEX_CACHE.lock().unwrap();
    if let Some(cached) = cache.get(latex) {
        return cached.clone();
    }

    let rendered = katex::render_with_opts(latex, KATEX_OPTS.as_ref()).unwrap();
    cache.insert(latex.to_string(), rendered.clone());
    rendered
}

#[derive(Debug)]
pub struct KatexBlock {
    pub latex: String,
    /// unused
    pub renderer: String,
}

impl NodeValue for KatexBlock {
    fn render(&self, node: &markdown_it::Node, fmt: &mut dyn markdown_it::Renderer) {
        fmt.text_raw(&format!(
            "<div class=\"katex-block\">{}</div>",
            render_latex(&self.latex)
        ));
    }
}

pub struct KatexBlockScanner;

impl BlockRule for KatexBlockScanner {
    fn run(
        state: &mut markdown_it::parser::block::BlockState,
    ) -> Option<(markdown_it::Node, usize)> {
        if state.get_line(state.line).get(0..1).unwrap_or_default() != "$$" {
            return None;
        }

        let start = state.line;
        let mut current_line = start;

        current_line += 1;

        while current_line < state.line_max {
            let line = state.get_line(current_line);
            if line.get(0..1).unwrap_or_default() == "$$" {
                let (latex, _) = state.get_lines(start + 1, current_line, 0, false);
                let node = KatexBlock {
                    latex,
                    renderer: "katex".to_string(),
                };
                return Some((Node::new(node), current_line - start + 1));
            }
            current_line += 1;
        }
        None
    }
}

pub fn add(md: &mut MarkdownIt) {
    md.block.add_rule::<KatexBlockScanner>();
}