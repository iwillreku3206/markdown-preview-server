pub fn generate_toc_html(markdown: String) -> String {
    let mut toc = String::new();
    let mut toc_depth = 0;
    let mut tags: Vec<String> = Vec::new();

    for line in markdown.lines() {
        if line.starts_with("```") && tags.last().unwrap_or(&"".to_string()) != "```" {
            tags.push("```".to_string());
            continue;
        }

        if line.starts_with("```") && tags.last().unwrap_or(&"".to_string()) == "```" {
            tags.pop();
            continue;
        }

        if line.starts_with('#') && tags.len() == 0 {
            let depth = line.chars().take_while(|c| *c == '#').count();
            let heading = line[depth..].trim();
            let heading_id = heading.to_lowercase().replace(" ", "-");
            if depth > toc_depth {
                toc.push_str(&"<ol>".repeat(depth - toc_depth));
            } else if depth < toc_depth {
                toc.push_str(&"</ol>".repeat(toc_depth - depth));
            }
            toc.push_str(&format!(
                "<li><a href=\"#{}\">{}</a></li>",
                heading_id, heading
            ));
            toc_depth = depth;
        }
    }
    toc.push_str(&"</ol>".repeat(toc_depth));
    toc
}
