use aoc::args::Puzzle;
use std::cmp::Ordering;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
pub enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn promote(&self) -> Self {
        match self {
            Packet::List(_) => panic!("Attempt to promote a list"),
            Packet::Value(n) => Packet::List(vec![Packet::Value(*n)]),
        }
    }

    fn dividers() -> [Packet; 2] {
        [
            Packet::List(vec![Packet::List(vec![Packet::Value(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Value(6)])]),
        ]
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        Self::cmp(self, other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(s), Self::Value(o)) => u32::cmp(s, o),
            (Self::List(sx), Self::List(ox)) => Vec::cmp(sx, ox),
            (Self::List(_), Self::Value(_)) => Self::cmp(self, &other.promote()),
            (Self::Value(_), Self::List(_)) => Self::cmp(&self.promote(), other),
        }
    }
}

mod packet_parser {
    use super::Packet;

    use nom::{
        branch::alt,
        character::complete::{char, digit1, space0},
        combinator::map_res,
        multi::separated_list0,
        sequence::{delimited, tuple},
        Finish, IResult,
    };

    fn value(input: &str) -> IResult<&str, Packet> {
        map_res(digit1, |res: &str| res.parse::<u32>().map(Packet::Value))(input)
    }

    fn values(input: &str) -> IResult<&str, Vec<Packet>> {
        separated_list0(tuple((char(','), space0)), alt((value, list)))(input)
    }

    fn list(input: &str) -> IResult<&str, Packet> {
        let (input, parts) = delimited(char('['), values, char(']'))(input)?;
        Ok((input, Packet::List(parts)))
    }

    pub fn parse(input: &str) -> Result<Packet, String> {
        match list(input).finish() {
            Ok((rest, packet)) if rest.is_empty() => Ok(packet),
            Ok((rest, _)) => Err(String::from("Junk trailing chars: ") + rest),
            Err(err) => Err(err.to_string()),
        }
    }

    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn convert_to_pairs<T>(mut vec: Vec<T>) -> Vec<(T, T)> {
        assert!(vec.len() % 2 == 0);

        let mut result = Vec::<(T, T)>::new();
        let mut drain = vec.drain(..);

        while drain.len() > 0 {
            result.push((drain.next().unwrap(), drain.next().unwrap()));
        }

        result
    }

    pub fn read_from_file(file: &File) -> Result<Vec<Packet>, String> {
        let mut packets = Vec::<Packet>::new();

        let mut lineno = 0;
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = aoc::io::read_line(&mut lines) {
            lineno += 1;

            if line.is_empty() {
                if lineno % 3 != 0 {
                    return Err(format!("{}: Expected empty line, got {}", lineno, line));
                }

                continue;
            }

            packets.push(line.parse::<Packet>()?);
        }

        Ok(packets)
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        packet_parser::parse(s)
    }
}

fn filter_correct(
    pairs: &'_ [(Packet, Packet)]
) -> impl Iterator<Item = (usize, &(Packet, Packet))> + '_ {
    pairs.iter().enumerate()
        .filter(|(_, (p1, p2))| p1 < p2)
        .map(|(i, v)| (i + 1, v))
}

fn correct_indices(pairs: &'_ [(Packet, Packet)]) -> impl Iterator<Item = usize> + '_ {
    filter_correct(pairs).map(|(i, _)| i)
}

fn decoder_key(mut packets: Vec<Packet>) -> usize {
    packets.extend(Packet::dividers());
    packets.sort();

    let div2 = packets.binary_search(&Packet::dividers()[0])
        .expect("Divider [[2]] not found");
    let div6 = packets.binary_search(&Packet::dividers()[1])
        .expect("Divider [[6]] not found");

    (div2 + 1) * (div6 + 1)
}

fn main() -> Result<(), String> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name).expect("Cannot open file");

    let packets = packet_parser::read_from_file(&file)?;

    println!("{}", match args.puzzle {
        Puzzle::P1 => {
            let pairs = packet_parser::convert_to_pairs(packets);
            correct_indices(&pairs).sum::<usize>()
        }
        Puzzle::P2 => decoder_key(packets),
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn examples() -> Vec<Packet> {
        use Packet::Value as V;
        use Packet::List as L;

        vec![
            L(vec![V(1), V(1), V(3), V(1), V(1)]),
            L(vec![V(1), V(1), V(5), V(1), V(1)]),

            L(vec![L(vec![V(1)]), L(vec![V(2), V(3), V(4)])]),
            L(vec![L(vec![V(1)]), V(4)]),

            L(vec![V(9)]),
            L(vec![L(vec![V(8), V(7), V(6)])]),

            L(vec![L(vec![V(4), V(4)]), V(4), V(4)]),
            L(vec![L(vec![V(4), V(4)]), V(4), V(4), V(4)]),

            L(vec![V(7), V(7), V(7), V(7)]),
            L(vec![V(7), V(7), V(7)]),

            L(vec![]),
            L(vec![V(3)]),

            L(vec![L(vec![L(vec![])])]),
            L(vec![L(vec![])]),

            L(vec![V(1), L(vec![V(2), L(vec![V(3),
                    L(vec![V(4), L(vec![V(5), V(6), V(7)])])])]),
                    V(8), V(9)]),
            L(vec![V(1), L(vec![V(2), L(vec![V(3),
                    L(vec![V(4), L(vec![V(5), V(6), V(0)])])])]),
                    V(8), V(9)]),
        ]
    }

    #[test]
    fn compare() {
        let examples = examples();

        assert!(examples[0] < examples[1]);
        assert!(examples[2] < examples[3]);
        assert!(examples[4] > examples[5]);
        assert!(examples[6] < examples[7]);
        assert!(examples[8] > examples[9]);
        assert!(examples[12] > examples[13]);
        assert!(examples[14] > examples[15]);
    }

    #[test]
    fn example1() {
        let examples = packet_parser::convert_to_pairs(examples());
        dbg!(&examples);
        let sum = correct_indices(&examples).sum::<usize>();

        assert_eq!(sum, 13);
    }

    #[test]
    fn example2() {
        assert_eq!(decoder_key(examples()), 140);
    }
}
