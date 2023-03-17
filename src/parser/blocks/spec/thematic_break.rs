use crate::parser::block::BlockType;
use regex::Regex;
/*
 *  From CommonMark Specification 0.29
 *  Under Section 4.1 "Thematic Breaks"
 *
 *  A line consisting of 0-3 spaces of indentation, followed
 *  by a sequence of three or more matching -, _, or * characters,
 *  each followed optionally by any number of spaces or tabs,
 *  forms a thematic break.
 *
 */

pub fn thematic_break() -> BlockType {
    return BlockType {
        id: "thematic_break".to_string(),
        start: regex::Regex::new(r"^[ ]{0,3}((([\-] {0,}){3,}|([_] {0,}){3,}|([\*] {0,}){3,})*)")
            .unwrap(),
        end: Regex::new(r"$").unwrap(),
        content: Regex::new(r"").unwrap(),
        exclude: [].to_vec(),
        template: "<hr />".to_string(),
    };
}
