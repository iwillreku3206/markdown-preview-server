// create function that converts markdown code (given as `String`) to an HTML table of contents, with nested lists for subheadings, and make each item link to #heading-1.2.3.4.5.6
pub fn generate_toc_html(markdown: String) -> String {
    let mut toc = String::new();
    let mut toc_depth = 0;
    for line in markdown.lines() {
        if line.starts_with("#") {
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