use std::collections::HashMap;
use std::time::Instant;
use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::StringOps;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
enum Condition {
    Operational = b'.',
    Damaged = b'#',
    Unknown = b'?',
}

struct HotSpringRow {
    conditions: Vec<Condition>,
    spec: Vec<u32>
}

struct Field {
    springs: Vec<HotSpringRow>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day12_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day12", Part::One), time_part1.elapsed().as_micros());
    println!("example (part2): {:?}", solve("inputs/day12_example", Part::Two));
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day12", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<u64> {
    let content = fs::read_to_string(path)?;
    let field = Field::parse(&content);

    Ok(match part {
        Part::One =>
            field.springs
                .iter()
                .map(|s| s.count_arrangements())
                .sum(),
        
        Part::Two =>
            field.springs
                .iter()
                .map(|s| s.unfold().count_arrangements())
                .sum()
    })
}

impl Field {

    fn parse(content: &str) -> Field {
        let springs = content.lines().map(|l| HotSpringRow::parse(&l).unwrap()).collect();
        Field { springs }
    }

}

impl HotSpringRow {

    fn parse(line: &str) -> Result<HotSpringRow> {
        let (cond_str, spec_str) = line.try_split_once(" ")?;
        Ok(HotSpringRow {
            conditions: cond_str.chars().map(|c| Condition::parse(c)).collect(),
            spec: spec_str.split(',').map(|ns| ns.parse().unwrap()).collect()
        })
    }

    fn count_arrangements(&self) -> u64 {
        count_arrangements(&self.conditions, &self.spec, &mut HashMap::new())
    }

    fn unfold(&self) -> HotSpringRow {
        let mut conditions = self.conditions.clone();
        for _ in 1 ..= 4 {
            conditions.push(Condition::Unknown);
            conditions.append(&mut self.conditions.clone());
        }
        
        let spec = self.spec.repeat(5);

        HotSpringRow { conditions, spec }
    }

}

impl core::fmt::Debug for HotSpringRow {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cond_str = String::from_iter(self.conditions.iter().map(|c| c.to_char()));
        writeln!(f, "{} {:?}", cond_str, self.spec)?;
        Ok(())
    }

}

impl Condition {

    fn parse(c: char) -> Condition {
        unsafe { std::mem::transmute(c as u8) }
    }
    
    fn to_char(&self) -> char {
        char::from(*self as u8)
    }

}

fn count_arrangements<'a, 'b>(
    mut conds: &'a [Condition],
    spec: &'b [u32],
    cache: &mut HashMap<(&'a [Condition], &'b [u32]), u64>
) -> u64 {
    // Get cached value if available
    if let Some(&total) = cache.get(&(conds, spec)) {
        return total;
    }

    // Skip any hotsprings that are operational
    while !conds.is_empty() && conds[0] == Condition::Operational {
        conds = &conds[1..];
    }

    // If we've reached the end, we must have zero specs to process
    if conds.is_empty() {
        return spec.is_empty() as u64;
    }

    // If we've no more specs to process, all the remaining must not be damaged
    if spec.is_empty() {
        return conds.iter().all(|c| *c != Condition::Damaged) as u64;
    }

    // If it's already clear there's no leftover space to fit all the damaged
    // hotsprings, give up
    let num_all_expected_damaged: u32 = spec.iter().sum();
    let num_all_min_expected_operational = spec.len() - 1;
    let min_space_to_fullfil_spec = num_all_expected_damaged + num_all_min_expected_operational as u32;
    if min_space_to_fullfil_spec > conds.len() as u32 {
        return 0;
    }
    
    // Otherwise, take the first value of the spec and try to apply it in different arrangements
    let mut num_arrangements = 0;
    let (&num_expected_damaged, rest_spec) = spec.split_first().unwrap();

    // One possible arrangement is to check if the immediate next conditions adhere to the spec
    let (cs_to_eval, rest_cs) = conds.split_at(num_expected_damaged as usize);
    if cs_to_eval.iter().all(|c| *c != Condition::Operational) {
        // If they do and there are no more conditions, count 1 if the spec is empty too
        // Otherwise only validate the remaining spec if we have at least one non-damaged space next
        if rest_cs.is_empty() {
            return rest_spec.is_empty() as u64;
        } else if rest_cs[0] != Condition::Damaged {
            let total = count_arrangements(&rest_cs[1..], rest_spec, cache);
            num_arrangements += *cache.entry((&rest_cs[1..], rest_spec)).or_insert(total);
        }
    }

    // Other possible arrangements might occur if we simply skip one cell, assuming it is empty
    if conds[0] != Condition::Damaged {
        let total = count_arrangements(&conds[1..], spec, cache);
        num_arrangements += *cache.entry((&conds[1..], spec)).or_insert(total);
    }
    
    return num_arrangements
}
