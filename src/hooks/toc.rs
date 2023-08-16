use std::char::REPLACEMENT_CHARACTER;

pub fn toc(input: String, markdown: String) -> String {
    let output = input.to_string().replace(
        &format!("{}toc{}", REPLACEMENT_CHARACTER, REPLACEMENT_CHARACTER),
        &crate::util::toc::generate_toc_html(markdown.to_string()),
    );
    return output.to_string();
}
