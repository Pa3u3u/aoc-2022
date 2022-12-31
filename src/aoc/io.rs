use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Read};

struct LineReader<T>
        where T: BufRead {
    lines: Lines<T>,
}

impl<T> Iterator for LineReader<T>
        where T: BufRead {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        read_line(&mut self.lines)
    }
}

pub fn lines(file: File) -> impl Iterator<Item = String> {
    let lines = BufReader::new(file).lines();
    LineReader { lines }
}

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
