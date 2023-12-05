use std::collections::HashSet;
use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::StringOps;

struct ScratchCard {
    winners: HashSet<u32>,
    mine: HashSet<u32>
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day04_example", Part::One));
    println!("input (part1): {:?}", solve("inputs/day04", Part::One));
    println!("example (part2): {:?}", solve("inputs/day04_example", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day04", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    let cards: Vec<ScratchCard> = content
        .lines()
        .map(|l| ScratchCard::parse(&l).unwrap())
        .collect();

    Ok(match part {
        Part::One =>
            cards.iter().map(|c| c.score()).sum(),
        
        Part::Two => {
            let mut card_counts = vec![0_u32; cards.len()];
            for (idx, c) in cards.iter().enumerate() {
                card_counts[idx] += 1;
                
                for next_idx in (idx + 1) .. (idx + 1 + c.num_won_matches() as usize) {
                    card_counts[next_idx] += card_counts[idx];
                }
            }
            card_counts.iter().sum()
        }
    })
}

impl ScratchCard {

    fn parse(line: &str) -> Result<ScratchCard> {
        let (_, card_content) = line.try_split_once(": ")?;
        let (winners_str, mine_str) = card_content.try_split_once(" | ")?;
        Ok(ScratchCard { 
            winners: winners_str.split_whitespace().map(|ws| ws.parse().unwrap()).collect(),
            mine: mine_str.split_whitespace().map(|ms| ms.parse().unwrap()).collect()
        })
    }

    fn num_won_matches(&self) -> u32 {
        self.winners.intersection(&self.mine).count() as u32
    }

    fn score(&self) -> u32 {
        let num_matches = self.num_won_matches();
        return if num_matches == 0 {
            0
        } else {
            2_i32.pow(num_matches - 1) as u32
        }
    }

}
