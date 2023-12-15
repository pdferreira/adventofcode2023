use std::{time::Instant, collections::HashMap};
use std::fs;
use adventofcode2023::{Result, Part};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, Hash, Eq)]
#[repr(u8)]
enum Tile {
    Empty = b'.',
    RoundRock = b'O',
    CubeRock = b'#'
}

struct Platform {
    tiles: Vec<Vec<Tile>>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day14_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day14", Part::One), time_part1.elapsed().as_micros());

    let time_part2_ex = Instant::now();
    println!("example (part2): {:?} ({} µs)", solve("inputs/day14_example", Part::Two), time_part2_ex.elapsed().as_micros());
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day14", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<usize> {
    let content = fs::read_to_string(path)?;
    let mut platform = Platform::parse(&content);

    Ok(match part {
        Part::One =>
            platform.tilt_north_and_calculate_load() as usize,
        
        Part::Two => {
            let mut cache = HashMap::new();
            // println!("Initial:\r\n{:?}", &platform);
            let mut i = 1;
            let mut limit = 1_000_000_000;
            let mut found_repetition = false;

            while i <= limit {
                platform.tilt_north();
                // println!("North {i}:\r\n{:?}", &platform);
                platform.tilt_west();
                // println!("West {i}:\r\n{:?}", &platform);
                platform.tilt_south();
                // println!("South {i}:\r\n{:?}", &platform);
                platform.tilt_east();
                // println!("East {i}:\r\n{:?}", &platform);

                if !found_repetition {
                    if let Some(prev_i) = cache.get(&platform.tiles) {
                        found_repetition = true;
                        println!("Found repeated state from cycle {prev_i} at cycle {i}, what now?");
                        let repeat_period = i - prev_i;
                        let missing_cycles = (1_000_000_000 - i) % repeat_period;
                        limit = i + missing_cycles - 1;
                        println!("Continuing only until cycle {limit}, which will match the end state");
                        continue;
                    } else {
                        cache.insert(platform.tiles.clone(), i);
                    }
                }
                i += 1;
            }
            platform.calculate_north_load()
        }
    })
}

impl Platform {

    fn parse<'a>(content: &str) -> Platform {
        let tiles = content
            .lines()
            .map(|l| l.chars().map(|c| Tile::parse(c)).collect())
            .collect();
        
        Platform { tiles }
    }

    fn tilt_north_and_calculate_load(&self) -> u32 {
        let mut stops = vec![0; self.tiles[0].len()];
        let mut load = 0;
        let max_load = self.tiles.len() as u32;

        for r in 0 .. self.tiles.len() {
            let ref row = self.tiles[r];
            for c in 0 .. row.len() {
                if row[c] == Tile::CubeRock {
                    stops[c] = r + 1;
                } else if row[c] == Tile::RoundRock {
                    load += max_load - (stops[c] as u32);
                    stops[c] += 1;
                }
            }
        }

        return load;
    }

    fn tilt_north(&mut self) {
        let mut stops = vec![0; self.tiles[0].len()];

        for r in 0 .. self.tiles.len() {
            for c in 0 .. self.tiles[r].len() {
                if self.tiles[r][c] == Tile::CubeRock {
                    stops[c] = r + 1;
                } else if self.tiles[r][c] == Tile::RoundRock {
                    self.tiles[r][c] = Tile::Empty;
                    self.tiles[stops[c]][c] = Tile::RoundRock;
                    stops[c] += 1;
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for r in 0 .. self.tiles.len() {
            let ref mut row = self.tiles[r];
            let mut stop = 0;
            for c in 0 .. row.len() {
                if row[c] == Tile::CubeRock {
                    stop = c + 1;
                } else if row[c] == Tile::RoundRock {
                    row[c] = Tile::Empty;
                    row[stop] = Tile::RoundRock;
                    stop += 1;
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        let mut stops = vec![self.tiles.len() - 1; self.tiles[0].len()];
        
        for r in (0 .. self.tiles.len()).rev() {
            for c in 0 .. self.tiles[r].len() {
                if self.tiles[r][c] == Tile::CubeRock && r > 0 {
                    stops[c] = r - 1;
                } else if self.tiles[r][c] == Tile::RoundRock {
                    self.tiles[r][c] = Tile::Empty;
                    self.tiles[stops[c]][c] = Tile::RoundRock;

                    if stops[c] > 0 {
                        stops[c] -= 1;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for r in 0 .. self.tiles.len() {
            let ref mut row = self.tiles[r];
            let mut stop = row.len() - 1;
            for c in (0 .. row.len()).rev() {
                if row[c] == Tile::CubeRock && c > 0 {
                    stop = c - 1;
                } else if row[c] == Tile::RoundRock {
                    row[c] = Tile::Empty;
                    row[stop] = Tile::RoundRock;

                    if stop > 0 {
                        stop -= 1;
                    }
                }
            }
        }
    }

    fn calculate_north_load(&self) -> usize {
        let max_load = self.tiles.len();
        let mut load = 0;

        for r in 0 .. self.tiles.len() {
            let total_round_rocks = self.tiles[r].iter().filter(|&&t| t == Tile::RoundRock).count();
            load += (max_load - r) * total_round_rocks;
        }

        return load;
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

impl core::fmt::Debug for Platform {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            writeln!(f, "{}", String::from_iter(row.iter().map(|t| t.to_char())))?;
        }
        Ok(())
    }

}
