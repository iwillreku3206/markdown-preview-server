use markdown_it::MarkdownIt;

use crate::markdown_extensions::katex::{self, KATEX_CACHE};

pub struct Parser {
    mdit: MarkdownIt,
}

impl Parser {
    pub fn new() -> Self {
        let mut mdit = MarkdownIt::new();
        let mdit_mut = &mut mdit;
        // TODO: insert code to customize plugin list

        markdown_it::plugins::cmark::add(mdit_mut);
        markdown_it::plugins::extra::add(mdit_mut);
        markdown_it::plugins::html::add(mdit_mut);
        markdown_it::plugins::sourcepos::add(mdit_mut);

        markdown_it_gfm::add(mdit_mut);
        markdown_it_front_matter::add(mdit_mut);
        markdown_it_footnote::add(mdit_mut);
        markdown_it_deflist::add(mdit_mut);

        katex::add(mdit_mut);

        Self { mdit }
    }

    pub fn parse(&self, text: &str) -> String {
        let ast = &self.mdit.parse(text);
        let html = ast.render();

        // cleanup caches
        let mut cache_lock = KATEX_CACHE.lock().unwrap();
        cache_lock.retain(|_, v| {
            v.1 -= 1;
            v.1 > 0
        });

        html
    }
}
