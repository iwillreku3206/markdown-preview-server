use regex::Regex;

pub struct BlockType {
    pub id: String,
    pub start: Regex,
    pub end: Regex,
    pub content: Regex,
    pub exclude: Vec<Regex>,
    pub template: String,
}

pub struct Block {
    pub block_type: BlockType,
    pub original: String,
    pub line_start: i64,
    pub line_end: i64,
    pub initital_transform: String,
    pub final_transformed: String,
    pub children: Vec<Block>,
}
