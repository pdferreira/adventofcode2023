use std::mem::transmute;
use std::ops::BitOrAssign;
use std::time::Instant;
use std::fs;
use adventofcode2023::{Result, Part};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
enum Tile {
    Empty = b'.',
    MirrorLeftRaising = b'/',
    MirrorLeftLowering = b'\\',
    HorizontalSplitter = b'-',
    VerticalSplitter = b'|',
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
enum Energized {
    No = 0x0,
    FromLeft = 0x1,
    FromRight = 0x2,
    FromHorizontal = (Energized::FromLeft as u8) | (Energized::FromRight as u8),
    FromUp = 0x4,
    FromDown = 0x8,
    FromVertical = (Energized::FromUp as u8) | (Energized::FromDown as u8),
}

struct Contraption {
    layout: Vec<Vec<Tile>>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day16_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day16", Part::One), time_part1.elapsed().as_micros());

    let time_part2_ex = Instant::now();
    println!("example (part2): {:?} ({} µs)", solve("inputs/day16_example", Part::Two), time_part2_ex.elapsed().as_micros());
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day16", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let contraption = Contraption::parse(&content);

    Ok(match part {
        Part::One => contraption.count_energized(0, 0, Energized::FromLeft),
        
        Part::Two => {
            let max_r = contraption.layout.len() - 1;
            let max_c = contraption.layout[0].len() - 1;

            let mut max_energized = 0;
            for r in 0 ..= max_r {
                let e = contraption.count_energized(r, 0, Energized::FromLeft);
                if e > max_energized {
                    max_energized = e;
                }

                let e = contraption.count_energized(r, max_c, Energized::FromRight);
                if e > max_energized {
                    max_energized = e;
                }
            }

            for c in 0 ..= max_c {
                let e = contraption.count_energized(0, c, Energized::FromUp);
                if e > max_energized {
                    max_energized = e;
                }

                let e = contraption.count_energized(max_r, c, Energized::FromDown);
                if e > max_energized {
                    max_energized = e;
                }
            }

            max_energized
        }
    })
}

impl Contraption {

    fn parse(content: &str) -> Contraption {
        let layout = content
            .lines()
            .map(|l| l.chars().map(|c| Tile::parse(c)).collect())
            .collect();

        Contraption { layout }
    }

    fn count_energized(&self, start_r: usize, start_c: usize, start_dir: Energized) -> u32 {
        let mut energized_cells = vec![vec![Energized::No; self.layout[0].len()]; self.layout.len()];
        self.simulate_beam(start_r, start_c, start_dir, &mut energized_cells);
        energized_cells
            .iter()
            .map(|e_row| e_row.iter().filter(|&&e| e != Energized::No).count() as u32)
            .sum()
    }

    fn simulate_beam(&self, start_r: usize, start_c: usize, start_dir: Energized, energized_cells: &mut Vec<Vec<Energized>>) {
        let max_r = self.layout.len() - 1;
        let max_c = self.layout[0].len() - 1;
        
        let mut r = start_r;
        let mut c = start_c;
        let mut dir = start_dir;

        loop {
            if energized_cells[r][c].includes(dir) {
                break;
            }
            energized_cells[r][c] |= dir;
            // println!("Processing r={r} c={c} with {dir:?}");

            match self.layout[r][c] {
                t if t == Tile::Empty 
                    || (t == Tile::HorizontalSplitter && Energized::FromHorizontal.includes(dir))
                    || (t == Tile::VerticalSplitter && Energized::FromVertical.includes(dir)) 
                => {
                    (r, c) = match dir {
                        Energized::FromDown if r > 0 => (r - 1, c),
                        Energized::FromUp if r < max_r => (r + 1, c),
                        Energized::FromLeft if c < max_c => (r, c + 1),
                        Energized::FromRight if c > 0 => (r, c - 1),
                        _ => break 
                    };
                },
                Tile::Empty => unreachable!("Empty should have been handled by the first case"),
                Tile::MirrorLeftLowering => {
                    (r, c, dir) = match dir {
                        Energized::FromDown if c > 0 => (r, c - 1, Energized::FromRight),
                        Energized::FromUp if c < max_c => (r, c + 1, Energized::FromLeft),
                        Energized::FromLeft if r < max_r => (r + 1, c, Energized::FromUp),
                        Energized::FromRight if r > 0 => (r - 1, c, Energized::FromDown),
                        _ => break
                    };
                },
                Tile::MirrorLeftRaising => {
                    (r, c, dir) = match dir {
                        Energized::FromDown if c < max_c => (r, c + 1, Energized::FromLeft),
                        Energized::FromUp if c > 0 => (r, c - 1, Energized::FromRight),
                        Energized::FromLeft if r > 0 => (r - 1, c, Energized::FromDown),
                        Energized::FromRight if r < max_r => (r + 1, c, Energized::FromUp),
                        _ => break
                    };
                },
                Tile::HorizontalSplitter => {
                    match dir {
                        Energized::FromLeft | Energized::FromRight => unreachable!("Should have been handled by the Empty case"),
                        Energized::FromUp | Energized::FromDown => {
                            if c > 0 {
                                self.simulate_beam(r, c - 1, Energized::FromRight, energized_cells);
                            }
                            if c < max_c {
                                self.simulate_beam(r, c + 1, Energized::FromLeft, energized_cells);
                            }
                        },
                        _ => unreachable!("Unexpected direction: {dir:?}")
                    }
                },
                Tile::VerticalSplitter => {
                    match dir {
                        Energized::FromDown | Energized::FromUp => unreachable!("Should have been handled by the Empty case"),
                        Energized::FromLeft | Energized::FromRight => {
                            if r > 0 {
                                self.simulate_beam(r - 1, c, Energized::FromDown, energized_cells);
                            }
                            if r < max_r {
                                self.simulate_beam(r + 1, c, Energized::FromUp, energized_cells);
                            }
                        },
                        _ => unreachable!("Unexpected direction: {dir:?}")
                    }
                }
            }
            
        }
        //println!("Stopping simulation at r={r}, c={c} with {dir:?}, as cell already is {:?}", energized_cells[r][c]);
    }

}

impl Tile {

    fn parse(c: char) -> Tile {
        unsafe { transmute(c as u8) }
    }

}

impl Energized {

    fn includes(self, other: Energized) -> bool {
        (self as u8) & (other as u8) == (other as u8)
    }

}

impl BitOrAssign for Energized {
    
    fn bitor_assign(&mut self, rhs: Self) {
        *self = unsafe { transmute((*self as u8) | (rhs as u8)) }
    }

}