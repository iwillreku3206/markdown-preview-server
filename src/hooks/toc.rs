pub fn toc(input: String) -> String {
    let output = input.to_string().replace(
        "[toc]",
        &crate::util::toc::generate_toc_html(input.to_string()),
    );
    return output.to_string();
}
