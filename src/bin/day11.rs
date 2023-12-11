use std::time::Instant;
use std::fs;
use adventofcode2023::{Result, Part};

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct SpaceImage {
    galaxy_coords: Vec<Coord>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day11_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day11", Part::One), time_part1.elapsed().as_micros());
    println!("example (part2): {:?}", solve("inputs/day11_example", Part::Two));
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day11", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<u64> {
    let content = fs::read_to_string(path)?;
    let mut image = SpaceImage::parse(&content);

    Ok(match part {
        Part::One => {
            image.expand_universe(2);
            calculate_total_distances(&image)
        }
        
        Part::Two => { 
            image.expand_universe(1_000_000);
            calculate_total_distances(&image)
        }
    })
}

impl SpaceImage {

    fn parse(content: &str) -> SpaceImage {
        let mut galaxy_coords = vec![];
        let mut empty_rows = vec![];
        let mut are_cols_empty = vec![]; 

        for (y, l) in content.lines().enumerate() {
            if are_cols_empty.is_empty() {
                are_cols_empty = vec![true; l.len()]
            }

            let mut found_galaxy = false;
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    found_galaxy = true;
                    are_cols_empty[x] = false;
                    galaxy_coords.push(Coord { x, y })
                }
            }

            if !found_galaxy {
                empty_rows.push(y);
            }
        }

        let empty_cols = are_cols_empty
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|(x, _)| x)
            .collect();

        SpaceImage { galaxy_coords, empty_rows, empty_cols }
    }

    fn expand_universe(&mut self, scale: u32) {
        for c in self.galaxy_coords.as_mut_slice() {
            c.x += (scale as usize - 1) * self.empty_cols.iter().take_while(|&&x| x < c.x).count();
            c.y += (scale as usize - 1) * self.empty_rows.iter().take_while(|&&y| y < c.y).count();
        }
    }

    fn calculate_distance(&self, g1: Coord, g2: Coord) -> u32 {
        let dx = g1.x.abs_diff(g2.x);
        let dy = g1.y.abs_diff(g2.y);

        return (dx + dy) as u32
    }

}

fn calculate_total_distances(image: &SpaceImage) -> u64 {
    // println!("{:?}", image);
    let mut total = 0_u64;
    for (i, &g1) in image.galaxy_coords.iter().enumerate() {
        for &g2 in &image.galaxy_coords[i+1 ..] {
            total += image.calculate_distance(g1, g2) as u64;
        }
    }
    total
}