/// Decodes special characters in the given string.
pub fn ipc_replace_special_chars(buf: &[u8]) -> String {
    let s = String::from_utf8_lossy(buf);
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(char) = chars.next() {
        match char {
            '\\' => match chars.next() {
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('t') => result.push('\t'),
                Some(other) => {
					result.push('\\');
					result.push(other);
				},
                None => result.push('\\'),
            },
            _ => result.push(char),
        }
    }

    return result;
}
