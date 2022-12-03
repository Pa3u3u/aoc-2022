use std::io::{BufRead, Lines};

pub fn read_line<T>(lines: &mut Lines<T>) -> Option<String>
        where T: BufRead {
    Some(lines.next()?.expect("Cannot read line"))
}
