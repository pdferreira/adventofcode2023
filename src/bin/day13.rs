use std::time::Instant;
use std::fs;
use adventofcode2023::{Result, Part};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Ash = b'.',
    Rocks = b'#',
}

struct Pattern {
    tiles: Vec<Vec<Tile>>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day13_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day13", Part::One), time_part1.elapsed().as_micros());
    println!("example (part2): {:?}", solve("inputs/day13_example", Part::Two));
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day13", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let mut lines_it = content.lines();
    let mut patterns = vec![];
    while let Some(pat) = Pattern::parse(&mut lines_it) {
        patterns.push(pat);
    }

    Ok(match part {
        Part::One =>
            summarize_mirrors(&patterns, false),
        
        Part::Two =>
            summarize_mirrors(&patterns, true)
    })
}

fn summarize_mirrors(patterns: &Vec<Pattern>, consider_smudges: bool) -> u32 {
    patterns
        .iter()
        .map(|p| {
            if let Some(n_rows) = p.get_row_mirror_span(consider_smudges) {
                // println!("Rows: {}", n_rows);
                100 * n_rows
            } else {
                let n_cols = p.transpose().get_row_mirror_span(consider_smudges).unwrap();
                // println!("Cols: {}", n_cols);
                n_cols
            }
        })
        .sum()
}

impl Pattern {

    fn parse<'a>(mut lines_it: impl Iterator<Item = &'a str>) -> Option<Pattern> {
        let mut tiles = vec![];
        while let Some(line) = lines_it.next().filter(|l| !l.is_empty()) {
            tiles.push(line.chars().map(|c| Tile::parse(c)).collect());
        }

        if tiles.is_empty() {
            None
        } else {
            Some(Pattern { tiles })
        }
    }

    fn get_row_mirror_span(&self, consider_smudges: bool) -> Option<u32> {
        for r in 0 .. self.tiles.len() {
            if self.is_mirror_row(r, consider_smudges) {
                return Some(r as u32 + 1)
            }
        }

        return None
    }

    fn is_mirror_row(&self, r: usize, consider_smudges: bool) -> bool {
        let mut up_r = r as i32;
        let mut down_r = r + 1;
        let mut has_found_smudge = false;

        while up_r >= 0 && down_r < self.tiles.len() {
            if self.tiles[up_r as usize] == self.tiles[down_r] {
                up_r -= 1;
                down_r += 1;
            } else {
                if consider_smudges && !has_found_smudge {
                    // If we haven't found the smudge, check if the only difference is a single tile
                    // and if so continue as if we had succeeded, simulating the scenario where this is the smudge
                    let mut diff_cols_it = self.tiles[up_r as usize]
                        .iter()
                        .zip(&self.tiles[down_r])
                        .enumerate()
                        .filter(|(_, (up_t, down_t))| **up_t != **down_t);

                    let first_diff_col = diff_cols_it.next().unwrap();
                    if diff_cols_it.next().is_none() {
                        // then it's unique
                        println!("Found mirror rows with unique difference: {} and {} at {}", up_r, down_r, first_diff_col.0);
                        has_found_smudge = true;
                        up_r -= 1;
                        down_r += 1;
                        continue;
                    }
                }

                return false;
            }
        }

        // It's a mirror row if we tested anything at all
        // and (iff looking for smudges) it has a smudge
        return up_r + 1 != down_r as i32
            && consider_smudges == has_found_smudge;
    }

    fn transpose(&self) -> Pattern {
        let mut tiles = vec![vec![]; self.tiles[0].len()];
        
        for row in &self.tiles {
            for c in 0 .. row.len() {
                tiles[c].push(row[c]);
            }
        }

        Pattern { tiles }
    }

}

impl Tile {

    fn parse(c: char) -> Tile {
        unsafe { std::mem::transmute(c as u8) }
    }
    
    fn to_char(&self) -> char {
        char::from(*self as u8)
    }

}

impl core::fmt::Debug for Tile {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }

}
