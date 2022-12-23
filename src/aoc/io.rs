use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

pub fn read_line<T>(lines: &mut Lines<T>) -> Option<String>
        where T: BufRead {
    Some(lines.next()?.expect("Cannot read line"))
}

pub fn read_file(file: File) -> Option<String> {
    let mut reader = BufReader::new(file);    
    let mut text = String::new();

    reader.read_to_string(&mut text).ok()?;
    Some(text)
}
