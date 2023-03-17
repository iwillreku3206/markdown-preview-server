use crate::parser::block::{Block, BlockType};
use crate::parser::frontmatter::read_frontmatter;

pub fn parse_file(file: &str, extensions: Vec<BlockType>) {
    let lines = str::split::<&str>(file, "\n");
    let line_arr: Vec<&str> = lines.clone().collect();
    let frontmatter = read_frontmatter(lines);

    let document_content = &line_arr[frontmatter
        .get(&"__frontmatter_end_line__".to_string())
        .unwrap_or(&"0".to_string())
        .to_string()
        .parse::<usize>()
        .unwrap_or_default()..line_arr.len() - 1];

    let blocks = Vec::<Block>::new();

    let mut current_block: Block;
    let mut current_block_content: String;
    let mut current_block_type: Option<&BlockType>;

    for current_line in 0..document_content.len() {
        // if there is no current block, scan for aa new block

        'line: for extension in &extensions {
            let matches_start = extension.start.is_match(document_content[current_line]);
            if !matches_start {
                continue;
            }
            let matches_exception = extension
                .exclude
                .iter()
                .map(|e| e.is_match(document_content[current_line]));

            for m in matches_exception {
                if m {
                    break 'line;
                }
            }

            current_block_type = Some(&extension);

            break;
        }
    }
}
