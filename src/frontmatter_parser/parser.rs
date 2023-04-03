use crate::frontmatter_parser::frontmatter::{read_frontmatter, remove_frontmatter};

use std::collections::HashMap;

pub struct DocumentWithFrontmatter {
    pub frontmatter: HashMap<String, String>,
    pub document_content: String,
}

pub fn parse_file_with_frontmatter(file: &str) -> DocumentWithFrontmatter {
    let lines = str::split::<&str>(file, "\n");
    let line_arr: Vec<&str> = lines.clone().collect();
    let frontmatter = read_frontmatter(lines);
    let document_content = remove_frontmatter(line_arr, frontmatter.clone());

    return DocumentWithFrontmatter {
        frontmatter: frontmatter,
        document_content: document_content,
    };
}
