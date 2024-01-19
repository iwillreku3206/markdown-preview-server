use std::collections::HashMap;

use markdown_it::MarkdownIt;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod editor;
pub mod parser;
pub mod web;

use crate::{args::Args, config::Config, viewer_connection::ViewerMap};

use self::editor::Editor;

pub struct Server {
    pub compiler: MarkdownIt,
    pub editors: HashMap<Uuid, Editor>,
    pub viewers: ViewerMap,
    pub config: Config,
    pub stdio: bool,
}

impl Server {
    pub fn new(args: &Args, config: Config) -> Self {
        Self {
            compiler: MarkdownIt::new(),
            viewers: RwLock::new(HashMap::new()),
            editors: HashMap::new(),
            config,
            stdio: args.stdio,
        }
    }
}
