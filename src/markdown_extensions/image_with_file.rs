//! Images
//!
//! `![image](<src> "title")`
//!
//! <https://spec.commonmark.org/0.30/#images>
use crate::patches::full_link;
use markdown_it::{
    parser::linkfmt::{LinkFormatter, MDLinkFormatter},
    MarkdownIt, Node, NodeValue, Renderer,
};
use url::Url;
use urlencoding::encode;

#[derive(Debug)]
pub struct Image {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for Image {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs = node.attrs.clone();
        attrs.push(("src", self.url.clone()));
        attrs.push(("alt", node.collect_text()));

        if let Some(title) = &self.title {
            attrs.push(("title", title.clone()));
        }

        fmt.self_close("img", &attrs);
    }
}

pub fn add(md: &mut MarkdownIt) {
    full_link::add_prefix::<'!', true>(md, |href, title| {
        let href = href.unwrap_or_default();
        let url = Url::parse(&href).unwrap();

        let final_href = match url.scheme() {
            "http" | "https" | "ftp" => href,
            "md-imagedir" => format!(
                "/imagedir?image={}",
                encode(&url.path().replace("/", "%2F"))
            ),
            _ => match MDLinkFormatter::new().validate_link(&href) {
                Some(_) => href,
                None => format!("/"),
            },
        };

        Node::new(Image {
            url: final_href,
            title,
        })
    });
}
