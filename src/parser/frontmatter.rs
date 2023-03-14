use std::str::Split;

pub fn read_frontmatter(document: Split<&str>) {
    document.clone().for_each(move |line| {
        println!("{}", line);
    })
}
