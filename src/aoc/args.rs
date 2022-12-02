use clap::{ArgGroup, Parser};

#[derive(Debug, Parser)]
#[command(group(ArgGroup::new("puzzle")))]
#[command(author, version, long_about = None)]
struct RawArguments {
    #[arg(short = '1', group = "puzzle")]
    p1: bool,

    #[arg(short = '2', group = "puzzle")]
    p2: bool,

    file_name: String,
}

#[derive(Debug)]
pub struct Arguments {
    pub puzzle: Puzzle,
    pub file_name: String,
}

#[derive(Debug)]
pub enum Puzzle {
    P1,
    P2,
}

impl Arguments {
    pub fn parse() -> Arguments {
        let raw = RawArguments::parse();

        // If ‹p2› is false, we always default to ‹p1›.
        let puzzle = if raw.p2 { Puzzle::P2 } else { Puzzle::P1 };

        Arguments {
            puzzle,
            file_name: raw.file_name,
        }
    }
}
