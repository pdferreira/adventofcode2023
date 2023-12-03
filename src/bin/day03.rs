use std::{fs, str::FromStr, cmp::{max, min}, collections::{HashMap, HashSet}};

struct EngineSchematic {
    rep: Vec<Vec<char>>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day03_example", /*part_1*/true));
    println!("input (part1): {:?}", solve("inputs/day03", /*part_1*/true));
    println!("example (part2): {:?}", solve("inputs/day03_example", /*part_1*/false));
    println!("input (part2): {:?}", solve("inputs/day03", /*part_1*/false));
}

fn solve(path: &str, part_1: bool) -> Result<u32, <u32 as FromStr>::Err> {
    let content = fs::read_to_string(path).unwrap();
    let schematic = EngineSchematic::parse(&content);
    return if part_1 { 
        Ok(schematic.get_part_numbers()?.iter().sum())
    } else {
        Ok(schematic.get_gear_ratios()?.iter().sum())
    }
    
}

impl EngineSchematic {
    fn parse(content: &str) -> EngineSchematic {
        EngineSchematic { 
            rep: content
                .lines()
                .map(|l| l.chars().collect())
                .collect() 
        }
    }

    fn get_part_numbers(&self) -> Result<Vec<u32>, <u32 as FromStr>::Err> {
        let mut part_numbers = Vec::<u32>::new();
        for r in 0 .. self.rep.len() {
            let row = &self.rep[r];
            let mut curr_n = String::new();
            let mut curr_is_part = false;
        
            for c in 0 .. row.len() {
                let v = row[c];
                if v.is_numeric() {
                    curr_n.push(v);
                    if !curr_is_part && !self.get_adjacent_symbols(r as i32, c as i32).is_empty() {
                        curr_is_part = true;
                    }
                } else {
                    if curr_is_part {
                        part_numbers.push(curr_n.parse()?);
                    }
                    curr_is_part = false;
                    curr_n.clear();
                }
            }

            if curr_is_part {
                part_numbers.push(curr_n.parse()?);
            }
        }
        return Ok(part_numbers);
    }

    fn get_adjacent_symbols(&self, curr_r: i32, curr_c: i32) -> Vec<(usize, usize)> {
        let mut symbols = Vec::new();
        for r in max(0, curr_r - 1) .. min(curr_r + 2, self.rep.len() as i32) {
            let row = &self.rep[r as usize];
            for c in max(0, curr_c - 1) .. min(curr_c + 2, row.len() as i32) {
                if r == curr_r && c == curr_c {
                    continue;
                }

                let v = row[c as usize];
                if v != '.' && !v.is_numeric() {
                    symbols.push((r as usize, c as usize));
                }
            }
        }

        return symbols;
    }

    fn get_gear_ratios(&self) -> Result<Vec<u32>, <u32 as FromStr>::Err> {
        let mut gears = HashMap::<(usize, usize), Vec<u32>>::new();
        for r in 0 .. self.rep.len() {
            let row = &self.rep[r];
            let mut curr_n = String::new();
            let mut curr_gears = HashSet::<(usize, usize)>::new();
        
            for c in 0 .. row.len() {
                let v = row[c];
                if v.is_numeric() {
                    curr_n.push(v);
                    self.get_adjacent_symbols(r as i32, c as i32)
                        .iter()
                        .filter(|s| self.rep[s.0][s.1] == '*')
                        .for_each(|s| { curr_gears.insert(*s) ; () });
                } else {
                    if !curr_gears.is_empty() {
                        let new_n = curr_n.parse()?;
                        for gear_pos in curr_gears.drain() {
                            gears
                                .entry(gear_pos)
                                .and_modify(|n| n.push(new_n))
                                .or_insert(vec![new_n]);
                        }
                    }
                    curr_n.clear();
                }
            }

            if !curr_gears.is_empty() {
                let new_n = curr_n.parse()?;
                for gear_pos in curr_gears.drain() {
                    gears
                        .entry(gear_pos)
                        .and_modify(|n| n.push(new_n))
                        .or_insert(vec![new_n]);
                }
            }
        }
        return Ok(
            gears
                .values()
                .filter(|ns| ns.len() == 2)
                .map(|ns| ns[0] * ns[1])
                .collect()
        );
    } 
}
