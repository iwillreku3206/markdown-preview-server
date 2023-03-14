use crate::parser::block::{Block, BlockType};
use crate::parser::frontmatter::read_frontmatter;

pub fn parse_file(file: &str, extensions: Vec<BlockType>) {
    let lines = str::split::<&str>(file, "\n");
    read_frontmatter(lines)
}
