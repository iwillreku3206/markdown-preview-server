use crate::parser::block::BlockType;
use regex::Regex;

/*
 * From CommonMark Specification 0.30
 * Under Section 4.8 "Paragraphs"
 *
 * A sequence of non-blank lines that cannot be interpreted as other kinds
 * of blocks forms a paragraph. The contents of the paragraph are the result
 * of parsing the paragraph’s raw content as inlines. The paragraph’s raw content
 * is formed by concatenating the lines and removing initial and final spaces or tabs.
 *
 * IMPORTANT NOTE: Make sure to place this extension at the end of the extension list, as this acts
 * as the "fallback" extension. for any content.
 */

pub fn paragraph() -> BlockType {
    return BlockType {
        id: "paragraph".to_string(),
        start: Regex::new(r".*").unwrap(),
        end: Regex::new(r".*").unwrap(),
        content: Regex::new("(.*)").unwrap(),
        exclude: [].to_vec(),
        template: "<p>{}</p>".to_string(),
    };
}
