mod sensor {
    use aoc::euclid::Point;
    use std::collections::BTreeSet;
    use std::fs::File;
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
    pub struct Range {
        pub min: isize,
        pub max: isize,
    }

    impl Range {
        pub fn new(min: isize, max: isize) -> Self {
            Range { min, max }
        }

        pub fn len(&self) -> usize {
            (self.max - self.min + 1) as usize
        }

        pub fn overlaps(&self, other: &Self) -> bool {
            self.min <= other.max && other.min <= self.max
        }

        pub fn nears(&self, other: &Self) -> bool {
            self.min == other.max + 1 || other.min == self.max + 1
        }

        pub fn join(&self, other: &Self) -> Self {
            assert!(self.overlaps(other) || self.nears(other));

            Range {
                min: std::cmp::min(self.min, other.min),
                max: std::cmp::max(self.max, other.max),
            }
        }

        pub fn punch(&self, other: &Self) -> (Option<Self>, Option<Self>) {
            let left = if self.min < other.min {
                Some(Range::new(self.min, other.min - 1))
            } else {
                None
            };

            let right = if self.max > other.max {
                Some(Range::new(other.max + 1, self.max))
            } else {
                None
            };

            (left, right)
        }
    }

    impl From<&(isize, isize)> for Range {
        fn from(r: &(isize, isize)) -> Self {
            Range { min: r.0, max: r.1 }
        }
    }

    impl From<(isize, isize)> for Range {
        fn from(r: (isize, isize)) -> Self {
            Range::from(&r)
        }
    }

    impl From<&Range> for core::ops::RangeInclusive<isize> {
        fn from(r: &Range) -> Self {
            r.min ..= r.max
        }
    }

    pub trait AxisProjection {
        fn axis(p: &Point) -> isize;
        fn transposed(p: &Point) -> isize;
    }

    pub struct XAxis;
    pub struct YAxis;

    impl AxisProjection for XAxis {
        fn axis(p: &Point) -> isize {
            p.x
        }

        fn transposed(p: &Point) -> isize {
            YAxis::axis(p)
        }
    }

    impl AxisProjection for YAxis {
        fn axis(p: &Point) -> isize {
            p.y
        }

        fn transposed(p: &Point) -> isize {
            XAxis::axis(p)
        }
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub struct Sensor {
        pub position: Point,
        pub beacon: Point,
        pub range: usize,
    }

    impl Sensor {
        pub fn new(position: Point, beacon: Point) -> Self {
            let range = (position.x - beacon.x).abs() + (position.y - beacon.y).abs();
            Self { position, beacon, range: range as usize }
        }

        pub fn cut<AP>(&self, pos: isize) -> Option<Range>
                where AP: AxisProjection {
            let center = AP::axis(&self.position);
            let anchor = AP::transposed(&self.position);

            let range = self.range as isize;

            if pos < anchor - range || anchor + range < pos {
                None
            } else {
                let shift = (pos - anchor).abs();
                Some(Range::new(center - range + shift, center + range - shift))
            }
        }

        #[allow(dead_code)]
        pub fn range<AP>(&self) -> Range
                where AP: AxisProjection {
            self.cut::<AP>(AP::transposed(&self.position)).unwrap()
        }

        #[allow(dead_code)]
        pub fn distance(&self, point: &Point) -> usize {
            ((self.position.x - point.x).abs() + (self.position.y - point.y).abs()) as usize
        }

        #[allow(dead_code)]
        pub fn contains(&self, point: &Point) -> bool {
            self.distance(point) <= self.range
        }
    }

    #[derive(Default)]
    pub struct Scan(pub Vec<Sensor>);

    impl Scan {
        pub fn new_from_file(file: File) -> Self {
            let mut sensors = Vec::<Sensor>::new();

            for line in aoc::io::lines(file) {
                if let Ok(sensor) = line.parse() {
                    sensors.push(sensor);
                }
            }

            Self(sensors)
        }

        pub fn cut<AP>(&self, position: isize) -> sparse_range::SparseRange
                where AP: AxisProjection {
            let mut builder = sparse_range::SparseRangeBuilder::new();

            for sensor in self.0.iter() {
                if let Some(range) = sensor.cut::<AP>(position) {
                    builder.add(&range);
                }
            }

            builder.build()
        }

        pub fn beacons<AP>(&self, position: isize) -> usize
                where AP: AxisProjection {
            let beacons = self.0.iter().filter(|sensor| AP::transposed(&sensor.beacon) == position)
                .map(|sensor| sensor.beacon)
                .collect::<BTreeSet<Point>>();

            beacons.len()
        }

        pub fn tiles_without_beacons<AP>(&self, position: isize) -> usize
                where AP: AxisProjection {
            self.cut::<AP>(position).len() - self.beacons::<AP>(position)
        }

        fn find_beacons(&self, xr: &Range, yr: &Range) -> Vec<Point> {
            let mut builder = SparseRangeBuilder::new();

            println!("Searching for candidate x axis");
            for y in core::ops::RangeInclusive::from(yr) {
                if y % 100_000 == 0 {
                    println!("  {:3} %", (100 * (y - yr.min)) / yr.len() as isize);
                }

                for range in self.cut::<XAxis>(y).holes_in_range(xr).0 {
                    builder.add(&range);
                }
            }

            let mut points = Vec::<Point>::new();
            let ranges = builder.build();

            println!("Found {} candidate ranges", ranges.0.len());
            println!("  Range size {}", ranges.len());

            for candidate_x_range in ranges.0 {
                for x in core::ops::RangeInclusive::from(&candidate_x_range) {
                    for range in self.cut::<YAxis>(x).holes_in_range(yr).0 {
                        for y in core::ops::RangeInclusive::from(&range) {
                            println!("  Found point [{}, {}]", x, y);
                            points.push((x, y).into());
                        }
                    }
                }
            }

            points
        }

        pub fn tuning_frequency(&self, area: &Range) -> Result<isize, String> {
            let beacons = self.find_beacons(area, area);
            let count = beacons.len();

            if count == 0 {
                Err("No beacon found".into())
            } else if count > 1 {
                Err(format!("{} beacons found", count))
            } else {
                Ok(beacons[0].x * 4_000_000 + beacons[0].y)
            }
        }
    }

    mod sparse_range {
        use super::Range;
        use std::collections::BTreeSet;

        #[derive(Debug)]
        pub struct SparseRange(pub Vec<Range>);

        impl SparseRange {
            pub fn len(&self) -> usize {
                self.0.iter().map(Range::len).sum::<usize>()
            }

            pub fn holes_in_range(&self, range: &Range) -> Self {
                let mut result = Vec::<Range>::new();
                let mut shard = *range;

                for filled in &self.0 {
                    if !filled.overlaps(&shard) {
                        continue;
                    }

                    let (left, right) = shard.punch(filled);
                    if let Some(left) = left {
                        result.push(left);
                    }

                    if let Some(right) = right {
                        shard = right;
                    } else {
                        break;
                    }
                }

                Self(result)
            }
        }

        #[derive(Default)]
        pub struct SparseRangeBuilder(BTreeSet<Range>);

        impl SparseRangeBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn add(&mut self, range: &Range) {
                self.0.insert(*range);
            }

            pub fn build(self) -> SparseRange {
                let mut result = Vec::<Range>::new();
                let mut cursor: Option<Range> = None;

                for range in self.0 {
                    if let Some(wip) = cursor {
                        cursor = if wip.overlaps(&range) || wip.nears(&range) {
                            Some(wip.join(&range))
                        } else {
                            result.push(wip);
                            Some(range)
                        }
                    } else {
                        cursor = Some(range)
                    }
                }

                if let Some(wip) = cursor {
                    result.push(wip);
                }

                SparseRange(result)
            }
        }
    }

    pub use sparse_range::SparseRangeBuilder;

    mod parser {
        use super::Sensor;

        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref SENSOR_RE: Regex =
                Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        }

        pub fn parse_sensor(line: &str) -> Option<Sensor> {
            let cap = SENSOR_RE.captures(line)?;

            let sensor_x = cap.get(1)?.as_str().parse::<isize>().unwrap();
            let sensor_y = cap.get(2)?.as_str().parse::<isize>().unwrap();
            let beacon_x = cap.get(3)?.as_str().parse::<isize>().unwrap();
            let beacon_y = cap.get(4)?.as_str().parse::<isize>().unwrap();

            Some(Sensor::new((sensor_x, sensor_y).into(), (beacon_x, beacon_y).into()))
        }
    }

    impl FromStr for Sensor {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            parser::parse_sensor(s).ok_or("Line does not match expected pattern")
        }
    }
}

use aoc::args::Puzzle;
use std::fs::File;

fn main() -> Result<(), String> {
    let args = aoc::args::Arguments::parse();
    let file = File::open(args.file_name).expect("Cannot open file");

    let scan = sensor::Scan::new_from_file(file);

    println!("{}", match args.puzzle {
        Puzzle::P1 => scan.tiles_without_beacons::<sensor::XAxis>(2_000_000) as isize,
        Puzzle::P2 => scan.tuning_frequency(&(0, 4_000_000).into())?,
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::sensor::*;

    #[test]
    fn sensor_cut() {
        let sensor = Sensor::new((2, 2).into(), (2, 6).into());

        assert_eq!(sensor.cut::<XAxis>(7), None);
        assert_eq!(sensor.cut::<XAxis>(6), Some((2, 2).into()));
        assert_eq!(sensor.cut::<XAxis>(5), Some((1, 3).into()));
        assert_eq!(sensor.cut::<XAxis>(4), Some((0, 4).into()));
        assert_eq!(sensor.cut::<XAxis>(3), Some((-1, 5).into()));
        assert_eq!(sensor.cut::<XAxis>(2), Some((-2, 6).into()));
        assert_eq!(sensor.cut::<XAxis>(1), Some((-1, 5).into()));
        assert_eq!(sensor.cut::<XAxis>(0), Some((0, 4).into()));
        assert_eq!(sensor.cut::<XAxis>(-1), Some((1, 3).into()));
        assert_eq!(sensor.cut::<XAxis>(-2), Some((2, 2).into()));
        assert_eq!(sensor.cut::<XAxis>(-3), None);
    }

    fn example_scan() -> Scan {
        Scan(vec![
            Sensor::new((2, 18).into(), (-2, 15).into()),
            Sensor::new((9, 16).into(), (10, 16).into()),
            Sensor::new((13, 2).into(), (15, 3).into()),
            Sensor::new((12, 14).into(), (10, 16).into()),
            Sensor::new((10, 20).into(), (10, 16).into()),
            Sensor::new((14, 17).into(), (10, 16).into()),
            Sensor::new((8, 7).into(), (2, 10).into()),
            Sensor::new((2, 0).into(), (2, 10).into()),
            Sensor::new((0, 11).into(), (2, 10).into()),
            Sensor::new((20, 14).into(), (25, 17).into()),
            Sensor::new((17, 20).into(), (21, 22).into()),
            Sensor::new((16, 7).into(), (15, 3).into()),
            Sensor::new((14, 3).into(), (15, 3).into()),
            Sensor::new((20, 1).into(), (15, 3).into()),
        ])
    }

    #[test]
    fn example1() {
        let scan = example_scan();
        assert_eq!(scan.tiles_without_beacons::<XAxis>(10), 26);
    }

    #[test]
    fn example2() {
        let scan = example_scan();
        assert_eq!(scan.tuning_frequency(&(0, 20).into()), Ok(56000011));
    }
}
