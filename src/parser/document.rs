use crate::parser::block::Block;
use std::collections::HashMap;

pub struct Document {
    pub frontmatter: HashMap<String, String>,
    pub blocks: Vec<Block>,
}
