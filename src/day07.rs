use aoc::args::Puzzle;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug)]
pub enum INode {
    File(usize),
    Directory(Directory),
}

#[derive(Debug, Default)]
pub struct Directory {
    entries: BTreeMap<String, INode>,
}

/* Rust's ‹std::path::Path› uses ‹OsString› and is just not very pleasant
 * to work with for this task. */
#[derive(Clone)]
struct Path {
    dirs: Vec<String>,
}

impl Path {
    pub fn new() -> Path {
        Path { dirs: Vec::new() }
    }

    pub fn change(&mut self, s: &str) -> Result<&mut Self, &'static str> {
        if s == "/" {
            self.dirs.clear();
        } else if s == ".." {
            self.dirs.pop();
        } else if !s.contains('/') {
            self.dirs.push(String::from(s));
        } else {
            return Err("Invalid path");
        }

        Ok(self)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.dirs.is_empty() {
            return write!(f, "/");
        }

        write!(f, "/{}", self.dirs.join("/"))
    }
}

struct DirBuilder {
    dir: Rc<Directory>,
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {
            dir: Rc::new(Directory::default()),
        }
    }

    pub fn get(&mut self, path: &Path) -> Result<FileBuilder, &'static str> {
        let mut dir: &mut Directory =
            Rc::get_mut(&mut self.dir).ok_or("Cannot borrow mutable")?;

        for comp in &path.dirs {
            dir = match dir.entries.entry(comp.to_string()) {
                Entry::Vacant(vacant) =>
                    match vacant.insert(INode::Directory(Directory::default())) {
                        INode::Directory(ndir) => ndir,
                        _ => panic!("BUG: Incorrect node inserted"),
                    },
                Entry::Occupied(occupied) =>
                    match occupied.into_mut() {
                        INode::Directory(ndir) => ndir,
                        _ => return Err("Not a directory"),
                    }
            }
        }

        Ok(FileBuilder::new(dir))
    }

    pub fn build(self) -> Rc<Directory> {
        self.dir
    }
}

struct FileBuilder<'a> {
    dir: &'a mut Directory,
}

impl<'a> FileBuilder<'a> {
    pub fn new(dir: &'a mut Directory) -> FileBuilder<'a> {
        FileBuilder { dir }
    }

    pub fn touch(self, file: &str, size: usize) -> Result<Self, &'static str> {
        match self.dir.entries.entry(file.to_string()) {
            Entry::Vacant(vacant) => vacant.insert(INode::File(size)),
            Entry::Occupied(_) => return Err("File exists"),
        };

        Ok(self)
    }

    pub fn mkdir(self, file: &str) -> Result<Self, &'static str> {
        match self.dir.entries.entry(file.to_string()) {
            Entry::Vacant(vacant) =>
                vacant.insert(INode::Directory(Directory::default())),
            Entry::Occupied(_) =>
                return Err("File exists"),
        };

        Ok(self)
    }
}

#[derive(Debug)]
enum Command {
    List,
    ChDir(String),
    MkDir(String),
    Touch(String, usize),
}

lazy_static! {
    static ref RE_CD: Regex = Regex::new(r"\$ cd (\S*)").unwrap();
    static ref RE_DIR: Regex = Regex::new(r"dir (\S*)").unwrap();
    static ref RE_FILE: Regex = Regex::new(r"(\d+) (\S*)").unwrap();
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "$ ls" {
            return Ok(Command::List);
        }

        if let Some(caps) = RE_CD.captures(s) {
            return Ok(Command::ChDir(caps.get(1).unwrap().as_str().into()));
        }

        if let Some(caps) = RE_DIR.captures(s) {
            return Ok(Command::MkDir(caps.get(1).unwrap().as_str().into()));
        }

        if let Some(caps) = RE_FILE.captures(s) {
            return Ok(Command::Touch(
                    caps.get(2).unwrap().as_str().into(),
                    caps.get(1).unwrap().as_str().parse().expect("Invalid number"))
            );
        }

        Err(String::from("Unknown line: ") + s)
    }
}

#[derive(Debug)]
pub struct Script(Vec<Command>);

impl Script {
    fn new() -> Script {
        Script(Vec::new())
    }

    fn run<'a>(&self, root: &'a mut DirBuilder) -> Result<&'a mut DirBuilder, &'static str> {
        let mut path = Path::new();
        let mut view = root.get(&path)?;

        for cmd in &self.0 {
            match cmd {
                Command::List => {}
                Command::ChDir(arg) => {
                    path.change(arg)?;
                    view = root.get(&path)?;
                }
                Command::Touch(arg, size) => {
                    view = view.touch(arg, *size)?;
                }
                Command::MkDir(arg) => {
                    view = view.mkdir(arg)?;
                }
            }
        }

        Ok(root)
    }

    fn read(file: &File) -> Result<Script, String> {
        let mut lines = BufReader::new(file).lines();
        let mut script = Script::new();

        while let Some(line) = aoc::io::read_line(&mut lines) {
            script.0.push(line.parse()?);
        }

        Ok(script)
    }
}

mod inspect {
    use super::*;

    fn _du(dir: &Directory, path: &Path, btm: &mut BTreeMap<String, usize>) -> usize {
        let mut size: usize = 0;

        for (name, inode) in &dir.entries {
            match inode {
                INode::File(s) => {
                    size += s;
                }

                INode::Directory(d) => {
                    let mut nd = path.clone();
                    nd.change(name).expect("BUG: Invalid path name");
                    size += _du(d, &nd, btm);
                }
            }
        }

        btm.insert(path.to_string(), size);
        size
    }

    pub fn disk_usage(root: &Directory) -> BTreeMap<String, usize> {
        let mut btm = BTreeMap::new();
        let path = Path::new();

        _du(root, &path, &mut btm);
        btm
    }

    pub fn sum_at_most(root: &Directory, size: usize) -> usize {
        disk_usage(root).values().filter(|m| m <= &&size).sum::<usize>()
    }

    pub fn find_candidate(root: &Directory, capacity: usize, required: usize) -> usize {
        let du = disk_usage(root);
        let used = du.get("/").expect("Root directory not contained");
        let mut sizes = du.values().cloned().collect::<Vec<usize>>();

        sizes.sort();

        sizes.iter().skip_while(|n| capacity - used + **n < required).copied()
            .next().expect("Value missing")
    }
}

fn main() -> IOResult<()> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name)?;

    let script = Script::read(&file).expect("Failed to read source");
    let mut builder = DirBuilder::new();

    script.run(&mut builder).expect("Failed to run script");

    println!("{}", match args.puzzle {
        Puzzle::P1 => inspect::sum_at_most(&builder.build(), 100_000),
        Puzzle::P2 => inspect::find_candidate(&builder.build(), 70_000_000, 30_000_000),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_sh() -> Script {
        Script(vec![
            Command::ChDir(String::from("/")),
            Command::MkDir(String::from("a")),
            Command::Touch(String::from("b.txt"), 14848514),
            Command::Touch(String::from("c.dat"), 8504156),
            Command::MkDir(String::from("d")),

            Command::ChDir(String::from("a")),
            Command::MkDir(String::from("e")),
            Command::Touch(String::from("f"), 29116),
            Command::Touch(String::from("g"), 2557),
            Command::Touch(String::from("h.lst"), 62596),

            Command::ChDir(String::from("e")),
            Command::Touch(String::from("i"), 584),

            Command::ChDir(String::from("..")),
            Command::ChDir(String::from("..")),

            Command::ChDir(String::from("d")),
            Command::Touch(String::from("j"), 4060174),
            Command::Touch(String::from("d.log"), 8033020),
            Command::Touch(String::from("d.ext"), 5626152),
            Command::Touch(String::from("k"), 7214296),
        ])
    }

    fn example_fs() -> Rc<Directory> {
        let mut builder = DirBuilder::new();
        example_sh().run(&mut builder).expect("Cannot construct example structure");
        builder.build()
    }

    #[test]
    fn example1() {
        assert_eq!(inspect::sum_at_most(&example_fs(), 100_000), 95_437);
    }

    #[test]
    fn example2() {
        assert_eq!(
            inspect::find_candidate(&example_fs(), 70_000_000, 30_000_000),
            24933642
        );
    }
}
