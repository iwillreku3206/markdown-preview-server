use std::collections::HashMap;

use markdown_it::MarkdownIt;
use uuid::Uuid;

pub mod editor;
pub mod state;
pub mod web;

use crate::config::Config;

use self::{editor::Editor, state::State};

pub struct Server {
    compiler: MarkdownIt,
    state: State,
    editors: HashMap<Uuid, Editor>,
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            compiler: MarkdownIt::new(),
            state: State::new(),
            editors: HashMap::new(),
            config,
        }
    }
}
