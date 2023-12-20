use adventofcode2023::{Result, Part, run};

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
enum Direction {
    Up = b'U',
    Down = b'D',
    Left = b'L',
    Right = b'R'
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    distance: u32,
    color: String
}

struct DigPlan {
    instructions: Vec<DigInstruction>,
}

struct LagoonSpan {
    num_rows: usize,
    num_cols: usize,
    start_r: usize,
    start_c: usize
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum DigSite {
    NotDug = b'.',
    Dug = b'#',
    NorthAndSouthBorder = b'|',
    NorthAndEastBorder = b'L',
    NorthAndWestBorder = b'J',
    WestAndEastBorder = b'-',
    SouthAndWestBorder = b'7',
    SouthAndEastBorder = b'F',
}

fn main() {
    run("inputs/day18_example", Part::One, solve);
    run("inputs/day18", Part::One, solve);

    run("inputs/day18_example", Part::Two, solve);
    run("inputs/day18", Part::Two, solve);
}

fn solve(content: &str, part: Part) -> Result<u64> {
    let mut dig_plan = DigPlan::parse(&content);

    Ok(match part {
        Part::One => dig_plan.get_lagoon_capacity() as u64,
        
        Part::Two => {
            for instr in dig_plan.instructions.as_mut_slice() {
                let color_len = instr.color.len();
                
                instr.distance = u32::from_str_radix(&instr.color[1 .. (color_len - 1)], 16)?;
                instr.direction = match &instr.color[color_len - 1 ..] {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => panic!()
                }
            }

            // Original solution for part 1 gave an OOM straightaway
            dig_plan.get_lagoon_capacity_by_maths()
        }
    })
}

impl DigPlan {

    fn parse(content: &str) -> DigPlan {
        let instructions = content
            .lines()
            .map(|l| DigInstruction::parse(&l).unwrap())
            .collect();

        DigPlan { instructions }
    }

    fn get_lagoon_span(&self) -> LagoonSpan {
        let mut max_width = 1;
        let mut max_height = 1;
        let mut min_width = 1;
        let mut min_height = 1;
        let mut curr_width = 1;
        let mut curr_height = 1;
        for instr in &self.instructions {
            match instr.direction {
                Direction::Down => curr_height += instr.distance as i32,
                Direction::Up => curr_height -= instr.distance as i32,
                Direction::Left => curr_width -= instr.distance as i32,
                Direction::Right => curr_width += instr.distance as i32
            }

            if curr_height > max_height {
                max_height = curr_height;
            }
            if curr_height < min_height {
                min_height = curr_height;
            }
            if curr_width > max_width {
                max_width = curr_width;
            }
            if curr_width < min_width {
                min_width = curr_width;
            }
        }

        LagoonSpan {
            num_rows: (max_height - min_height + 1) as usize,
            num_cols: (max_width - min_width + 1) as usize,
            start_r: (min_height - 1).abs() as usize,
            start_c: (min_width - 1).abs() as usize
        }
    }

    fn get_lagoon_capacity(&self) -> u32 {
        let LagoonSpan { num_rows, num_cols, start_r, start_c } = self.get_lagoon_span();

        let mut r = start_r;
        let mut c = start_c;
        let mut prev_dir = self.instructions.last().unwrap().direction;
        let mut dig_sites = vec![vec![DigSite::NotDug; num_cols]; num_rows];
        println!("Start {start_r} and {start_c} and {num_rows} and {num_cols}");

        for instr in &self.instructions {
            dig_sites[r][c] = DigSite::from_directions(prev_dir, instr.direction);

            match instr.direction {
                Direction::Down => {
                    for i in (r + 1) .. (r + instr.distance as usize) {
                        dig_sites[i][c] = DigSite::NorthAndSouthBorder;
                    }
                    
                    r += instr.distance as usize;
                    //dig_sites[r][c] = Direction::Up as u8;
                },
                Direction::Up => {
                    for i in (r - instr.distance as usize + 1) .. r {
                        dig_sites[i][c] = DigSite::NorthAndSouthBorder;
                    }
                    r -= instr.distance as usize;
                },
                Direction::Left => {
                    // lagoon[r][(c - instr.distance as usize) .. c].fill(true);
                    for col in &mut dig_sites[r][(c - instr.distance as usize + 1) .. c] {
                        *col = DigSite::WestAndEastBorder;
                    }
                    c -= instr.distance as usize;
                },
                Direction::Right => {
                    // lagoon[r][c .. (c + instr.distance as usize)].fill(true);
                    for col in &mut dig_sites[r][(c + 1) .. (c + instr.distance as usize)] {
                        *col = DigSite::WestAndEastBorder;
                    }
                    c += instr.distance as usize;
                }
            }
            prev_dir = instr.direction;
            // println!("Now at {r} and {c} after executing {:?}", instr)
        }

        // Fix start
        dig_sites[start_r][start_c] = DigSite::from_directions(prev_dir, self.instructions[0].direction);

        // for row in &dig_sites {
        //     println!("{}", row.iter().map(|&ds| ds.to_char().to_string()).collect::<Vec<_>>().join(""));
        // }
        // println!();

        // Dig inside
        // Initialize a slightly bigger matrix to store if the corners are outside or inside.
        // We know the outer corners all around are necessarily outside, so those get marked right away.
        let outside_len = num_rows + 1;
        let outside_row_len = num_cols + 1;

        let mut sample_outside = vec![None; outside_row_len];
        sample_outside[0] = Some(true);
        sample_outside[outside_row_len - 1] = Some(true);
        
        let mut outside = vec![sample_outside; outside_len];
        outside[0] = vec![Some(true); outside_row_len];
        outside[outside_len - 1] = vec![Some(true); outside_row_len];

        // For each map position, keep updating the southwest and southeast corners depending on the shape
        // of the boundary. At the same time, use the northeast corner info to understand if undug positions
        // should be marked as outside or inside.
        for r in 0 .. num_rows {
            if dig_sites[r][0] == DigSite::NotDug {
                outside[r + 1][0] = Some(true);
                outside[r + 1][1] = Some(true);
            }

            for c in 0 .. dig_sites[r].len() {
                let is_northwest_outside = outside[r][c];

                match dig_sites[r][c] {
                    DigSite::NorthAndEastBorder => {
                        outside[r + 1][c] = is_northwest_outside;
                        outside[r + 1][c + 1] = is_northwest_outside;
                    },
                    DigSite::NorthAndSouthBorder => {
                        outside[r + 1][c] = is_northwest_outside;
                        outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                    },
                    DigSite::NorthAndWestBorder => {
                        outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                        outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                    },
                    DigSite::WestAndEastBorder => {
                        outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                        outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                    },
                    DigSite::SouthAndEastBorder => {
                        outside[r + 1][c] = is_northwest_outside;
                        outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                    },
                    DigSite::SouthAndWestBorder => {
                        outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                        outside[r + 1][c + 1] = is_northwest_outside;
                    },
                    DigSite::NotDug => {
                        outside[r + 1][c] = is_northwest_outside;
                        outside[r + 1][c + 1] = is_northwest_outside;

                        if !outside[r][c].unwrap() {
                            dig_sites[r][c] = DigSite::Dug;
                        }
                    },
                    DigSite::Dug => panic!("Not expecting to find Dug tiles at all")
                }
            }
        }

        // for row in &dig_sites {
        //     println!("{}", row.iter().map(|&ds| ds.to_char().to_string()).collect::<Vec<_>>().join(""));
        // }

        dig_sites
            .iter()
            .map(|row| row.iter().filter(|&&ds| ds != DigSite::NotDug).count() as u32)
            .sum()
    }

    fn get_lagoon_capacity_by_maths(&self) -> u64 {
        let mut points = vec![(0, 0)]; 
        let mut r = 0;
        let mut c = 0;
        let mut total_dist = 0;

        for instr in &self.instructions {
            total_dist += instr.distance as i64;
            match instr.direction {
                Direction::Right => c += instr.distance as i64,
                Direction::Left => c -= instr.distance as i64,
                Direction::Down => r += instr.distance as i64,
                Direction::Up => r -= instr.distance as i64
            }
            points.push((c, r));
        }

        let mut area = 0; 
        for i in 1 .. points.len() {
            area += points[i-1].0 * points[i].1 - points[i].0 * points[i-1].1;
        }

        return 1 + (total_dist / 2 + area.abs() / 2) as u64;
    }

}

impl DigInstruction {

    fn parse(line: &str) -> Option<DigInstruction> {
        let mut words_it = line.split_whitespace();
        let direction = Direction::parse(&words_it.next()?);
        let distance = words_it.next()?.parse().ok()?;
        let color = words_it.next()?
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string();
        Some(DigInstruction { direction, distance, color })
    }

}

impl Direction {

    fn parse(s: &str) -> Direction {
        let first_char = s.chars().take(1).collect::<Vec<_>>()[0];
        unsafe { std::mem::transmute(first_char as u8) }
    }

}

impl DigSite {

    fn from_directions(prev: Direction, curr: Direction) -> DigSite {
        match (prev, curr) {
            (Direction::Down, Direction::Down) => DigSite::NorthAndSouthBorder,
            (Direction::Down, Direction::Left) => DigSite::NorthAndWestBorder,
            (Direction::Down, Direction::Right) => DigSite::NorthAndEastBorder,
            (Direction::Up, Direction::Up) => DigSite::NorthAndSouthBorder,
            (Direction::Up, Direction::Left) => DigSite::SouthAndWestBorder,
            (Direction::Up, Direction::Right) => DigSite::SouthAndEastBorder,
            (Direction::Left, Direction::Left) => DigSite::WestAndEastBorder,
            (Direction::Left, Direction::Up) => DigSite::NorthAndEastBorder,
            (Direction::Left, Direction::Down) => DigSite::SouthAndEastBorder,
            (Direction::Right, Direction::Right) => DigSite::WestAndEastBorder,
            (Direction::Right, Direction::Up) => DigSite::NorthAndWestBorder,
            (Direction::Right, Direction::Down) => DigSite::SouthAndWestBorder,
            _ => panic!()
        }
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        char::from(*self as u8)
    }

}