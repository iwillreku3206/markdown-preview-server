pub fn generate_toc_html(markdown: String) -> String {
    let mut toc = String::new();
    let mut header_count = vec![0; 6]; // count H1-H6 headers
    let mut parent_levels = vec![]; // track parent header levels
    let mut levels = vec![]; // track current nesting level

    for _line in markdown.lines() {
        let mut line = _line.to_string();
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count();
            if level >= 1 && level <= 6 {
                header_count[level - 1] += 1;
                let header_number = header_count[0..level]
                    .iter()
                    .map(|&c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                let header_id = format!("header-{}-{}", level, header_count[level - 1]);
                let header_text = line[level + 1..].trim();

                // determine parent header level
                let mut parent_level = 0;
                if let Some(last_parent_level) = parent_levels.last() {
                    if *last_parent_level < level {
                        parent_level = *last_parent_level;
                    }
                }
                parent_levels.push(level);

                // determine nesting level
                let mut nesting_level = levels.len();
                if nesting_level > 0 && levels[nesting_level - 1] >= level {
                    nesting_level -= 1;
                    while nesting_level > 0 && levels[nesting_level - 1] >= level {
                        nesting_level -= 1;
                    }
                }
                levels.truncate(nesting_level);
                levels.push(level);

                // generate table of contents entry
                let mut indent = String::new();
                for _ in 0..nesting_level {
                    indent.push_str("<li><ol>");
                }
                toc += &format!(
                    "{}<li><a href=\"#{0}\">{1} {2}</a></li>\n",
                    indent, header_number, header_text
                );
                for _ in 0..nesting_level {
                    toc.push_str("</ol></li>");
                }
                line.replace_range(..level + 1, &format!("<h{0} id=\"{1}\">", level, header_id));
                line.push_str(&format!("</h{}>", level));
            }
        }
    }

    if toc.is_empty() {
        return String::new();
    }

    format!("<ol>{}</ol>", toc)
}
