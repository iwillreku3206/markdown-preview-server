//! Links
//!
//! `![link](<to> "stuff")`
//!
//! <https://spec.commonmark.org/0.30/#links>
//!
//! Modified to include custom data attributes for custom link behavior.
use markdown_it::generics::inline::full_link;
use markdown_it::{MarkdownIt, Node, NodeValue, Renderer};

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for Link {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        let url = self.url.trim();

        if url.starts_with("md://") {
            attrs.push(("href", "".to_string()));
            attrs.push(("data-path", url.trim_start_matches("md://").to_string()));
        }
        attrs.push(("href", self.url.clone()));

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children);
        fmt.close("a");
    }
}

pub fn add(md: &mut MarkdownIt) {
    full_link::add::<false>(md, |href, title| {
        Node::new(Link {
            url: href.unwrap_or_default(),
            title,
        })
    });
}
