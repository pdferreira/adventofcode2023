use std::cmp::min;
use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::{StringOps, parse_sequence};

#[allow(dead_code)]
enum Part2Solution {
    RunEverySeedThroughPart1, // Runs in < 1h
    UseRangesAllWayThrough // Much more efficient
}

const PART2_SOLUTION_TO_USE: Part2Solution = Part2Solution::UseRangesAllWayThrough;

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    length: u64
}

struct Mapping {
    src: Range,
    dst_start: u64
}

struct Map {
    #[allow(dead_code)]
    name: String,
    mappings: Vec<Mapping>
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day05_example", Part::One));
    println!("input (part1): {:?}", solve("inputs/day05", Part::One));
    println!("example (part2): {:?}", solve("inputs/day05_example", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day05", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<u64> {
    let content = fs::read_to_string(path)?;
    let almanac = Almanac::parse(&content)?;

    Ok(match (part, PART2_SOLUTION_TO_USE) {
        (Part::One, _) =>
            almanac.seeds
                .iter()
                .map(|seed| almanac.where_to_plant(*seed))
                .min()
                .ok_or("No seeds")?,
        
        (Part::Two, Part2Solution::RunEverySeedThroughPart1) =>
            almanac.seeds
                .chunks(2)
                .flat_map(|c| c[0] .. c[0] + c[1])
                .map(|seed| almanac.where_to_plant(seed))
                .min()
                .ok_or("No seeds")?,
        
        (Part::Two, Part2Solution::UseRangesAllWayThrough) =>
            almanac.seeds
                .chunks(2)
                .map(|c| Range { start: c[0] as u64, length: c[1] as u64 })
                .flat_map(|r| almanac.where_to_plant_range(r))
                .map(|r| r.start)
                .min()
                .ok_or("No seeds or ranges are miscalculated")?
    })
}

impl Range {

    fn end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn contains(&self, num: u64) -> bool {
        self.start <= num && num <= self.end()
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end() && other.start <= self.end()
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        if !self.overlaps(other) {
            None
        } else if self.start >= other.start {
            // 0123456789        345
            // [  ( ]   )  ==>   ( ]
            Some(Range {
                start: self.start,
                length: min(self.length, other.end() - self.start + 1)
            })
        } else {
            // 0123456789        4567
            // (   [  ) ]  ==>   [  )
            Some(Range {
                start: other.start,
                length: min(other.length, self.end() - other.start + 1)
            })
        }
    }

    fn except(&self, other: &Range) -> Vec<Range> {
        let mut res = vec![];
        if self.start < other.start && self.end() >= other.start {
            // 0123456789        0123456789
            // ( [  )   ]   ==>  ()
            //     ([]  )   ==>      |
            res.push(Range { 
                start: self.start, 
                length: other.start - self.start
            });
        }

        if self.end() > other.end() && self.start <= other.end() {
            // 0123456789         0123456789
            // [   ( ]  )    ==>         ( )
            //     ([]  )     ==>        ( )
            //   ( []  )     ==>        ( )
            let new_start = other.end() + 1;
            res.push(Range {
                start: new_start,
                length: self.end() - new_start + 1 
            })
        }

        return res
    }

    fn shift(&self, delta: i64) -> Range {
        Range {
            start: ((self.start as i64) + delta) as u64,
            length: self.length
        }
    }

}

impl Mapping {

    fn parse(line: &str) -> Result<Mapping> {
        let ns: Vec<u32> = parse_sequence(&line);
        Ok(Mapping {
            src: Range {
                start: ns[1] as u64,
                length: ns[2] as u64
            },
            dst_start: ns[0] as u64,
        })
    }

    fn convert(&self, num: u64) -> Option<u64> {
        if self.src.contains(num) {
            Some(self.dst_start + (num - self.src.start))
        } else {
            None
        }
    }

    fn convert_range(&self, num_range: Range) -> Option<(Range, Vec<Range>)> {
        /* From
               src: (start: 4, len: 2), dst_start: 50, range: (start: 2, len: 7)

           We can conclude that numbers 4-5 match this MapRange and thus become 50-51
           and that numbers 2-3 and 6-8 don't match this MapRange and thus need to proceed
        */ 
        match self.src.intersect(&num_range) {
            None => None,
            Some(r) => {
                let converted = r.shift((self.dst_start as i64) - (self.src.start as i64));
                let remaining = num_range.except(&r);
                // Return the range that is changed and the (max 2) sub-ranges of the input
                // that fallback to any other MapRanges or pass as is
                Some((converted, remaining))
            }
        }
    }

}

impl Map {

    fn parse<'a>(line_it: &mut impl Iterator<Item = &'a str>) -> Result<Map> {
        let (name, _) = line_it
            .next()
            // if it's the new-line skip it and try again
            .filter(|s| !s.is_empty())
            .or_else(|| line_it.next())
            // then check result and split once to get the name without the suffix
            .ok_or("No header for map")?
            .try_split_once(" ")?;

        let mut ranges = vec![];
        while let Some(range_str) = line_it.next().filter(|s| !s.is_empty()) {
            ranges.push(Mapping::parse(&range_str)?);
        }
        Ok(Map { name: name.to_string(), mappings: ranges })
    }

    fn convert(&self, num: u64) -> u64 {
        self.mappings
            .iter()
            .find_map(|r| r.convert(num))
            .unwrap_or(num)
    }

    fn convert_range(&self, num_range: Range) -> Vec<Range> {
        let mut to_process = vec![num_range];
        let mut result = vec![];
        for map_range in &self.mappings {
            let mut new_to_process = vec![];
            for r in to_process {
                if let Some((converted, mut remaining)) = map_range.convert_range(r) {
                    // Append to the result all those converted
                    result.push(converted);

                    // Add to the next round the ranges remaining
                    new_to_process.append(&mut remaining)
                } else {
                    new_to_process.push(r);
                }
            }
            to_process = new_to_process;
        }

        // If we have unmapped ranges, they go to the result as is
        result.append(&mut to_process);
        return result;
    }

}

impl Almanac {

    fn parse(content: &str) -> Result<Almanac> {
        let mut line_it = content.lines().peekable();
        let seed_line = line_it.next().ok_or("no first line")?;

        let seeds = parse_sequence(&seed_line["seeds: ".len() ..]);

        let mut maps = vec![];
        while line_it.peek().is_some() {
            let map = Map::parse(&mut line_it)?;
            maps.push(map);
        }

        Ok(Almanac { seeds, maps })
    }

    fn where_to_plant(&self, seed: u32) -> u64 {
        self.maps
            .iter()
            .fold(seed as u64, |num, map| map.convert(num))
    }

    fn where_to_plant_range(&self, seed_range: Range) -> Vec<Range> {
        self.maps
            .iter()
            .fold(
                vec![seed_range], 
                |rs, map| rs
                    .iter()
                    .flat_map(|r| map.convert_range(*r))
                    .collect())
    }

}
