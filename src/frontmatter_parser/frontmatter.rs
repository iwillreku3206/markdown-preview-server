use std::collections::HashMap;
use std::str::Split;
use yaml_rust::YamlLoader;

pub fn read_frontmatter(document: Split<&str>) -> HashMap<String, String> {
    let lines: Vec<&str> = document.clone().collect();

    let mut end_line: usize = 0;

    if !(lines[0] == "+++" || lines[0] == "---") {
        return HashMap::new();
    }

    let mut i: usize = 1;

    while i < lines.len() {
        if lines[i] == lines[0] {
            end_line = i + 1;
            i = lines.len();
        }
        i += 1;
    }

    if end_line == 0 {
        return HashMap::new();
    }

    let mut frontmatter_map = HashMap::new();

    let frontmatter = &lines[1..end_line - 1].join("\n");
    let frontmatter_parsed = YamlLoader::load_from_str(frontmatter).unwrap_or_default();

    for fm in frontmatter_parsed {
        let hash = fm.as_hash();
        for h in hash {
            for j in h {
                frontmatter_map.insert(
                    j.0.as_str().unwrap_or_default().to_string(),
                    j.1.as_str().unwrap_or_default().to_string(),
                );
            }
        }
    }
    frontmatter_map.insert("__frontmatter_end_line__".to_string(), end_line.to_string());

    return frontmatter_map;
}

pub fn remove_frontmatter(line_arr: Vec<&str>, frontmatter: HashMap<String, String>) -> String {
    let document_content = &line_arr[frontmatter
        .get(&"__frontmatter_end_line__".to_string())
        .unwrap_or(&"0".to_string())
        .to_string()
        .parse::<usize>()
        .unwrap_or_default()..line_arr.len() - 1];

    return document_content.join("\n");
}
