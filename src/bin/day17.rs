use std::collections::HashMap;
use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use adventofcode2023::{Result, Part, run};

struct HeatMap {
    map: Vec<Vec<u8>>
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct CrucibleCell {
    r: usize,
    c: usize,
    dr: isize,
    dc: isize,
    same_dir_budget: u8
}

fn main() {
    run("inputs/day17_example", Part::One, solve);
    run("inputs/day17", Part::One, solve);

    run("inputs/day17_example", Part::Two, solve);
    run("inputs/day17_example2", Part::Two, solve);
    run("inputs/day17", Part::Two, solve);
}

fn solve(content: &str, part: Part) -> Result<u32> {
    let heat_map = HeatMap::parse(&content);

    let start_r = 0;
    let start_c = 0;
    let target_r = heat_map.num_rows() - 1;
    let target_c = heat_map.num_cols() - 1;

    Ok(match part {
        Part::One => heat_map.min_heat(start_r, start_c, target_r, target_c, 0, 3),
        
        Part::Two => heat_map.min_heat(start_r, start_c, target_r, target_c, 4, 10)
    })
}

impl HeatMap {

    fn parse(content: &str) -> HeatMap {
        let map = content
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();

        HeatMap { map }
    }

    fn num_rows(&self) -> usize {
        self.map.len()
    }

    fn num_cols(&self) -> usize {
        self.map[0].len()
    }
    
    fn min_heat(&self, sr: usize, sc: usize, tr: usize, tc: usize, min_same_dir: u8, max_same_dir: u8) -> u32 {
        let num_rows = self.num_rows();
        let num_cols = self.num_cols();
        let min_required_budget = max_same_dir - min_same_dir;

        // To ensure we explore the variations coming from all 4 directions + budget
        // store the min heats for each and evaluate at the end
        let mut min_heats = vec![vec![HashMap::new(); num_cols]; num_rows];
        let mut to_visit = PriorityQueue::<CrucibleCell, Reverse<u32>>::new();

        min_heats[sr][sc].insert((0, 0, max_same_dir), 0);
        to_visit.push(CrucibleCell { r: sr, c: sr, dr: 0, dc: 0, same_dir_budget: max_same_dir }, Reverse(0));

        while let Some((cell, Reverse(min_heat))) = to_visit.pop() {
            for (dr, dc) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                // If invalid indexes, ignore
                let nr = cell.r as isize + dr;
                let nc = cell.c as isize + dc;
                if nr < 0 || (nr as usize) >= num_rows || nc < 0 || (nc as usize) >= num_cols {
                    continue;
                }

                // If turning back, ignore
                if (dr == 0 && dc == -cell.dc) || (dc == 0 && dr == -cell.dr) {
                    continue;
                }

                // If going in the same direction, check and update budget
                let same_dir_budget: u8;
                if dr == cell.dr && dc == cell.dc {
                    if cell.same_dir_budget == 0 {
                        continue;
                    }
                    same_dir_budget = cell.same_dir_budget - 1;
                } else if cell.same_dir_budget > min_required_budget && (cell.dr != 0 || cell.dc != 0) {
                    // If not going in the same direction but the budget minimum was not spent, ignore
                    continue;
                } else {
                    same_dir_budget = max_same_dir - 1;
                }

                let nr = nr as usize;
                let nc = nc as usize;
                let n_min_heat = min_heat + self.map[nr][nc] as u32;
                if let Some(old_n_min_heat) = min_heats[nr][nc].get_mut(&(dr, dc, same_dir_budget)) {
                    if *old_n_min_heat > n_min_heat {
                        *old_n_min_heat = n_min_heat;
                        to_visit.push_increase(CrucibleCell { r: nr, c: nc, dr, dc, same_dir_budget }, Reverse(n_min_heat));
                    }
                } else {
                    min_heats[nr][nc].insert((dr, dc, same_dir_budget), n_min_heat);
                    to_visit.push(CrucibleCell { r: nr, c: nc, dr, dc, same_dir_budget }, Reverse(n_min_heat));
                }
            }
            // println!("{:?} with heat {}", cell, min_heat);
            // println!("{:?}", to_visit.clone().into_sorted_vec());
        }

        // print_min_heats(&min_heats);
        *min_heats[tr][tc]
            .iter()
            .filter(|((_, _, budget), _)| *budget <= min_required_budget)
            .map(|(_, h)| h)
            .min()
            .unwrap()
    }

}

#[allow(dead_code)]
fn print_min_heats(matrix: &Vec<Vec<HashMap<(isize, isize, u8), u32>>>) {
    for row in matrix {
        println!("{}", row
            .iter()
            .map(|ns| ns
                .iter()
                .map(|(_, h)| h)
                .min()
                .map(|n| n.to_string())
                .unwrap_or("?".to_string())
            )
            .collect::<Vec<String>>()
            .join("\t"));
    }
}