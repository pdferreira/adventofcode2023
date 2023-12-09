use std::collections::HashMap;
use std::fs;
use num::integer::lcm;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::StringOps;

enum Direction {
    Left,
    Right
}

type Node<'a> = &'a str;

struct Documents<'a> {
    instructions: Vec<Direction>,
    network: HashMap<Node<'a>, (Node<'a>, Node<'a>)>
}

fn main() {
    println!("example1 (part1): {:?}", solve("inputs/day08_example1", Part::One));
    println!("example2 (part1): {:?}", solve("inputs/day08_example2", Part::One));
    println!("input (part1): {:?}", solve("inputs/day08", Part::One));
    println!("example3 (part2): {:?}", solve("inputs/day08_example3", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day08", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<u128> {
    let content = fs::read_to_string(path)?;
    let documents = Documents::parse(&content)?;

    Ok(match part {
        Part::One =>
            documents.count_steps("AAA", |n| n == "ZZZ") as u128,
        
        Part::Two => {
            // Simplified solution since the input ends up not having multiple paths, etc.
            let cycle_steps: Vec<_> = documents.network
                .keys()
                .filter(|n| n.ends_with('A'))
                .map(|n| documents.count_steps(*n, |curr_n| curr_n.ends_with('Z')))
                .collect();

            cycle_steps.iter().fold(1 as u128, |a, b| lcm(a, *b as u128))
        }
    })
}

#[derive(Debug, Clone)]
struct CycleStats(usize, u32, u32);

impl<'a> Documents<'a> {

    fn parse(content: &str) -> Result<Documents> {
        let mut line_it = content.lines();
        let instr_str = line_it.next().ok_or("No instructions")?;
        let instructions = instr_str
            .chars()
            .map(|c| if c == 'L' { Direction::Left } else { Direction::Right })
            .collect();

        line_it.next(); // skip empty line

        let mut network = HashMap::new();
        for line in line_it {
            let (node, edges_str) = line.try_split_once(" = ")?;
            network.insert(node, (&edges_str[1 .. 4], &edges_str[6 .. 9]));
        }

        Ok(Documents { instructions, network })
    }

    fn count_steps(&self, src: Node, is_end: impl Fn(Node) -> bool) -> u32 {
        let mut curr_node = src;
        let mut num_steps = 0;
        let mut instr_idx = 0;

        while !is_end(curr_node) {
            let (left, right) = self.network[curr_node];
            curr_node = match self.instructions[instr_idx] {
                Direction::Left => left,
                Direction::Right => right
            };
            instr_idx = (instr_idx + 1) % self.instructions.len();
            num_steps += 1;
        }

        return num_steps;
    }

    // The general problem may include multiple paths to ends, some of which enter cycles
    // others not, which requires knowing 1) which instruction reaches the end, 
    // 2) when the cycle starts and 3) what's the cycle period. That's what this function
    // attempts to find.
    //
    // In practice the input has none of that complexity, just a single path that cycles
    // consistently.
    #[allow(dead_code)]
    fn get_ending_stats(&self, src: Node<'a>, dst_suffix: &str) -> Vec<CycleStats> {
        let mut end_path_lens: HashMap<(Node, usize, bool), u32> = HashMap::new();
        let mut end_path_stats: Vec<CycleStats> = vec![];

        let mut curr_node = src;
        let mut instr_idx = 0;
        let mut curr_step = 0;

        loop {
            if curr_node.ends_with(dst_suffix) {
                if end_path_lens.contains_key(&(curr_node, instr_idx, true)) {
                    // Only returning if it's the second time we found the cycle, to ensure we collected all of them
                    return end_path_stats;
                } else if let Some(&prev_step) = end_path_lens.get(&(curr_node, instr_idx, false)) {
                    end_path_lens.insert((curr_node, instr_idx, true), curr_step);

                    let cycle_len = curr_step - prev_step;
                    end_path_stats.push(CycleStats(instr_idx, prev_step % cycle_len, cycle_len));
                } else {
                    end_path_lens.insert((curr_node, instr_idx, false), curr_step);
                }
            }

            let (left, right) = self.network[curr_node];
            curr_node = match self.instructions[instr_idx] {
                Direction::Left => left,
                Direction::Right => right
            };
            instr_idx = (instr_idx + 1) % self.instructions.len();
            curr_step += 1;
        }
    }

    #[allow(dead_code)]
    fn get_common_cycles(&self) -> Vec<Vec<CycleStats>> {
        let starter_nodes: Vec<_> = self.network.keys().filter(|n| n.ends_with("A")).collect();
        
        let mut common_cycles = vec![vec![]; self.instructions.len()];
        for node in starter_nodes.as_slice() {
            let stats = self.get_ending_stats(node, "Z");
            println!("{} -> {:?}", node, stats);

            // Store all cycles that are hit in the same instructions
            for s in stats {
                common_cycles[s.0].push(s);
            }
        }

        // Keep only cycles that are common to all starter nodes
        return common_cycles
            .as_slice()
            .iter()
            .filter(|c| c.len() == starter_nodes.len())
            .map(|c| c.clone())
            .collect();
    }

}