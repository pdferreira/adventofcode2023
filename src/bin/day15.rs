use std::time::Instant;
use std::fs;
use adventofcode2023::{Result, Part};

fn main() {
    println!("example (part1): {:?}", solve("inputs/day15_example", Part::One));
    let time_part1 = Instant::now();
    println!("input (part1): {:?} ({} µs)", solve("inputs/day15", Part::One), time_part1.elapsed().as_micros());

    let time_part2_ex = Instant::now();
    println!("example (part2): {:?} ({} µs)", solve("inputs/day15_example", Part::Two), time_part2_ex.elapsed().as_micros());
    let time_part2 = Instant::now();
    println!("input (part2): {:?} ({} µs)", solve("inputs/day15", Part::Two), time_part2.elapsed().as_micros());
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let init_sequence = content
        .split(',')
        .map(|str| str.trim());

    Ok(match part {
        Part::One =>
            init_sequence
                .map(|s| hash(s))
                .sum(),
        
        Part::Two => {
            const DEFAULT_BOX: Vec<(String, u32)> = vec![];
            let mut boxes = [DEFAULT_BOX; 256];
            
            'steps: for step in init_sequence {
                if step.ends_with('-') {
                    let label = &step[.. step.len() - 1];
                    let b = hash(label) as usize;
                    if let Some(i) = boxes[b].iter().position(|(l, _)| l == label) {
                        boxes[b].remove(i);
                    }
                } else {
                    let (label, focal_len_str) = step.split_once('=').unwrap();
                    let focal_len = focal_len_str.parse().unwrap();
                    
                    let b = hash(label) as usize;
                    for (l, fl) in boxes[b].iter_mut() {
                        if l == label {
                            *fl = focal_len;
                            continue 'steps;
                        }
                    }

                    boxes[b].push((label.to_string(), focal_len));
                }
            }
            
            boxes
                .iter()
                .enumerate()
                .map(|(i, lenses)| lenses
                    .iter()
                    .enumerate()
                    .map(|(j, (_, f))| (i as u32 + 1) * (j as u32 + 1) * f)
                    .sum::<u32>()
                )
                .sum() 
        }
    })
}

fn hash(s: &str) -> u32 {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as u32;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}