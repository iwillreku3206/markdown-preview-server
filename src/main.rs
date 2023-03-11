pub mod web;
use web::web_start;

fn main() -> Result<(), String> {
    //println!("Hello, world!");
    let res = markdown::to_html_with_options(
        include_str!(
            "/home/rek/Coding/rust/markdown-preview-server/23-2-10.pandoc"
        )
        /*"```js
                const test = \"test\"
                // This is a test file
                console.log(test)
            ```
            ```mermaid
                graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
            ```
            "*/,
        &markdown::Options {
            compile: markdown::CompileOptions::default(),
            parse: {
                markdown::ParseOptions {
                    constructs: markdown::Constructs {
                        attention: true,
                        autolink: true,
                        block_quote: true,
                        character_escape: true,
                        character_reference: true,
                        code_indented: true,
                        code_fenced: true,
                        code_text: true,
                        definition: true,
                        frontmatter: true,
                        gfm_autolink_literal: true,
                        gfm_footnote_definition: true,
                        gfm_label_start_footnote: true,
                        gfm_strikethrough: true,
                        gfm_table: true,
                        gfm_task_list_item: true,
                        hard_break_escape: true,
                        hard_break_trailing: true,
                        heading_atx: true,
                        heading_setext: true,
                        html_flow: true,
                        html_text: true,
                        label_start_image: true,
                        label_start_link: true,
                        label_end: true,
                        list_item: true,
                        math_flow: true,
                        math_text: true,
                        mdx_esm: true,
                        mdx_expression_flow: true,
                        mdx_expression_text: true,
                        mdx_jsx_flow: true,
                        mdx_jsx_text: true,
                        thematic_break: true,
                    },
                    ..markdown::ParseOptions::default()
                }
            },
        },
    )?;

    println!("{}", res);
    Ok(())
    //web_start();
}
