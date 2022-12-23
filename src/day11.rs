use aoc::args::Puzzle;
use std::cell::RefCell;
use std::collections::{BinaryHeap, VecDeque};
use std::fs::File;
use std::str::FromStr;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

type WorryLevel = isize;

#[derive(Debug)]
struct MonkeyTest {
    divisor: WorryLevel,
    if_true: MonkeyID,
    if_false: MonkeyID,
}

impl MonkeyTest {
    fn new(divisor: WorryLevel, if_true: MonkeyID, if_false: MonkeyID) -> Self {
        Self { divisor, if_true, if_false }
    }

    fn exec(&self, n: WorryLevel) -> MonkeyID {
        if n % self.divisor == 0 { self.if_true } else { self.if_false }
    }
}

type MonkeyID = usize;

#[derive(Debug)]
enum Operation<T> {
    Add(T),
    Mul(T),
    Pow,
}

impl Operation<WorryLevel> {
    fn eval(&self, k: &WorryLevel) -> WorryLevel {
        match self {
            Self::Add(n) => k + n,
            Self::Mul(n) => k * n,
            Self::Pow => k * k,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<WorryLevel>,
    op: Operation<WorryLevel>,
    test: MonkeyTest,
    inspected: usize,
}

trait WorryLevelManagement {
    fn manage(&self, n: WorryLevel) -> WorryLevel;
}

#[derive(Default)]
struct DroppingWLM;

impl WorryLevelManagement for DroppingWLM {
    fn manage(&self, n: WorryLevel) -> WorryLevel {
        n / 3
    }
}

#[derive(Default)]
struct NoWLM;

impl WorryLevelManagement for NoWLM {
    fn manage(&self, n: WorryLevel) -> WorryLevel {
        n
    }
}

#[derive(Default)]
struct LCMWLM {
    lcm: WorryLevel,
}

impl LCMWLM {
    fn new(party: &MonkeyParty) -> Self {
        let lcm = party.0.iter().map(|mref| mref.borrow().test.divisor)
            .reduce(num::integer::lcm).expect("Empty Monkey party");

        Self { lcm }
    }
}

impl WorryLevelManagement for LCMWLM {
    fn manage(&self, n: WorryLevel) -> WorryLevel {
        n % self.lcm
    }
}

impl Monkey {
    fn new(items: &[WorryLevel], op: Operation<WorryLevel>, test: MonkeyTest) -> Self {
        let mut deq: VecDeque<WorryLevel> = VecDeque::with_capacity(items.len());
        deq.extend(items.iter());
        Self { items: deq, op, test, inspected: 0 }
    }

    fn throw<WLM: WorryLevelManagement>(&mut self, wlm: &WLM) -> Option<(WorryLevel, MonkeyID)> {
        let mut item = self.items.pop_front()?;
        self.inspected += 1;
        item = wlm.manage(self.op.eval(&item));
        Some((item, self.test.exec(item)))
    }

    fn receive(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }

    fn active(&self) -> usize {
        self.inspected
    }
}

#[derive(Debug, Default)]
struct MonkeyParty(Vec<RefCell<Monkey>>);

impl MonkeyParty {
    #[allow(dead_code)]
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self(monkeys.into_iter().map(RefCell::new).collect())
    }

    fn add(&mut self, mid: MonkeyID, monkey: Monkey) -> Result<(), String> {
        if mid != self.0.len() {
            return Err(format!("Invalid ID: Expected {}, got {}", self.0.len(), mid))
        }

        self.0.push(monkey.into());
        Ok(())
    }

    fn turn<WLM: WorryLevelManagement>(&self, monkey: &mut Monkey, wlm: &WLM) {
        while let Some((item, target)) = monkey.throw::<WLM>(wlm) {
            self.0[target].borrow_mut().receive(item);
        }
    }

    fn round<WLM: WorryLevelManagement>(&mut self, wlm: &WLM) {
        for (_, mc) in self.0.iter().enumerate() {
            self.turn(&mut mc.borrow_mut(), wlm);
        }
    }

    fn rounds<WLM: WorryLevelManagement>(&mut self, n: usize, wlm: &WLM) {
        for _ in 0 .. n {
            self.round(wlm)
        }
    }

    fn business(&self) -> usize {
        let mut heap: BinaryHeap<usize> = BinaryHeap::new();
        heap.extend(self.0.iter().map(|monkey| monkey.borrow().active()));
        heap.iter().take(2).product()
    }
}

#[derive(Parser)]
#[grammar = "day11.pest"]
struct PartyParser;

impl PartyParser {
    fn uw<R>(
        p: Result<pest::iterators::Pairs<R>, pest::error::Error<R>>
    ) -> Result<pest::iterators::Pairs<R>, String>
            where R: pest::RuleType {
        p.map_err(|r| format!("{}", r))
    }

    fn uw_span2num<R, T>(p: &mut pest::iterators::Pair<R>) -> T
            where R: pest::RuleType,
                  T: FromStr,
                  <T as FromStr>::Err: std::fmt::Debug {
        p.as_str().parse::<T>().unwrap()
    }

    fn uw_rule2num<R, T>(p: &mut pest::iterators::Pairs<R>) -> T
            where R: pest::RuleType,
                  T: FromStr,
                  <T as FromStr>::Err: std::fmt::Debug {
        Self::uw_span2num(&mut p.next().unwrap())
    }

    fn build_monkey(pair: pest::iterators::Pair<Rule>) -> (usize, Monkey) {
        let mut id: MonkeyID = 0;
        let mut items: Vec<WorryLevel> = Vec::default();
        let mut op: Operation<WorryLevel> = Operation::Add(0);
        let mut test: MonkeyTest = MonkeyTest::new(0, 0, 0);

        for p in pair.into_inner() {
            match p.as_rule() {
                Rule::monkey_id => {
                    id = Self::uw_rule2num(&mut p.into_inner());
                }
                Rule::items => {
                    for mut item in p.into_inner() {
                        items.push(Self::uw_span2num(&mut item));
                    }
                }
                Rule::operation => {
                    let inner = p.into_inner().next().unwrap();
                    op = match inner.as_rule() {
                        Rule::addition => {
                            Operation::Add(Self::uw_rule2num(&mut inner.into_inner()))
                        }
                        Rule::multiplication => {
                            Operation::Mul(Self::uw_rule2num(&mut inner.into_inner()))
                        }
                        Rule::power => {
                            Operation::Pow
                        }
                        _ => panic!("Unreachable")
                    }
                }
                Rule::test => {
                    let mut parts = p.into_inner();

                    let div: WorryLevel = Self::uw_rule2num(&mut parts);
                    let ift: MonkeyID = Self::uw_rule2num(&mut parts);
                    let iff: MonkeyID = Self::uw_rule2num(&mut parts);
                    
                    test = MonkeyTest::new(div, ift, iff);
                }
                _ => {}
            }
        }

        (id, Monkey::new(&items, op, test))
    }

    fn run(str: &str) -> Result<MonkeyParty, String> {
        let monkeys = Self::uw(Self::parse(Rule::monkey_party, str))?
            .next().unwrap().into_inner().into_iter().next().unwrap();

        let mut party = MonkeyParty::default();
        for monkey in monkeys.into_inner() {
            let (id, monkey) = Self::build_monkey(monkey);
            party.add(id, monkey)?;
        }

        Ok(party)
    }
}

fn main() -> Result<(), String> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name).expect("Cannot open file");
    let text = aoc::io::read_file(file).expect("Cannot read file");

    let mut mp = match PartyParser::run(&text) {
        Err(s) => {
            println!("{}", &s);
            return Err("Parser failed".into());
        }

        Ok(mp) => mp,
    };

    match args.puzzle {
        Puzzle::P1 => mp.rounds(20, &DroppingWLM::default()),
        Puzzle::P2 => mp.rounds(10000, &LCMWLM::new(&mp)),
    }

    println!("{}", mp.business());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> MonkeyParty {
        MonkeyParty::new(vec![
            Monkey::new(&[79, 98], Operation::Mul(19), MonkeyTest::new(23, 2, 3)),
            Monkey::new(&[54, 65, 75, 74], Operation::Add(6), MonkeyTest::new(19, 2, 0)),
            Monkey::new(&[79, 60, 97], Operation::Pow, MonkeyTest::new(13, 1, 3)),
            Monkey::new(&[74], Operation::Add(3), MonkeyTest::new(17, 0, 1)),
        ])
    }

    #[test]
    fn throws() {
        let example = example();
        let wlm = DroppingWLM::default();

        assert_eq!(example.0[0].borrow_mut().throw(&wlm), Some((500, 3)));
        assert_eq!(example.0[0].borrow_mut().throw(&wlm), Some((620, 3)));
        assert_eq!(example.0[0].borrow_mut().throw(&wlm), None);

        assert_eq!(example.0[1].borrow_mut().throw(&wlm), Some((20, 0)));
        assert_eq!(example.0[1].borrow_mut().throw(&wlm), Some((23, 0)));
        assert_eq!(example.0[1].borrow_mut().throw(&wlm), Some((27, 0)));
        assert_eq!(example.0[1].borrow_mut().throw(&wlm), Some((26, 0)));
        assert_eq!(example.0[1].borrow_mut().throw(&wlm), None);
    }

    #[test]
    fn round() {
        let mut example = example();

        example.round(&DroppingWLM::default());

        assert_eq!(example.0[0].borrow().items, [20, 23, 27, 26]);
        assert_eq!(example.0[1].borrow().items, [2080, 25, 167, 207, 401, 1046]);
        assert_eq!(example.0[2].borrow().items, []);
        assert_eq!(example.0[3].borrow().items, []);
    }

    #[test]
    fn example_business() {
        let mut example = example();
        example.rounds(20, &DroppingWLM::default());
        assert_eq!(example.business(), 10605);
    }

    #[test]
    fn example_business2() {
        let mut example = example();
        example.rounds(10000, &LCMWLM::new(&example));
        assert_eq!(example.business(), 2713310158);
    }
}
