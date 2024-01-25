use std::{collections::HashMap, sync::Arc};

use markdown_it::MarkdownIt;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod editor;
pub mod parser;
pub mod web;

use crate::{
    args::Args, config::Config, editor_connection::EditorConnection, viewer_connection::ViewerMap,
};

use self::editor::Editor;

pub struct Server {
    pub compiler: MarkdownIt,
    pub editors: HashMap<Uuid, Editor>,
    pub viewers: ViewerMap,
    pub config: Config,
    pub stdio: bool,
    pub io: Arc<dyn EditorConnection>,
}

impl Server {
    pub fn new(args: &Args, config: Config, io: Arc<dyn EditorConnection>) -> Self {
        Self {
            compiler: MarkdownIt::new(),
            viewers: RwLock::new(HashMap::new()),
            editors: HashMap::new(),
            config,
            stdio: args.stdio,
            io,
        }
    }
}
