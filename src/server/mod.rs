use std::collections::HashMap;

use markdown_it::MarkdownIt;
use uuid::Uuid;

pub mod editor;
pub mod parser;
pub mod state;
pub mod web;

use crate::config::Config;

use self::{editor::Editor, state::State};

pub struct Server {
    pub compiler: MarkdownIt,
    pub state: State,
    pub editors: HashMap<Uuid, Editor>,
    pub config: Config,
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
    pub fn _test_print_config(&self) {
        println!("port: {:?}", self.config.web.port);
    }
}
