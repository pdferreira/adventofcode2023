use std::cell::RefCell;
use std::collections::BTreeSet;
use adventofcode2023::{Result, Part, run};

fn main() {
    run("inputs/day21_example", Part::One, |c, p| solve(c, p, 6));
    run("inputs/day21", Part::One, |c, p| solve(c, p, 64));

    // run("inputs/day21_example", Part::Two, solve);
    // run("inputs/day21", Part::Two, solve);
}


fn solve(content: &str, part: Part, num_steps: u8) -> Result<usize> {
    let garden: Vec<Vec<_>> = content
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let (start_r, start_c) = garden
        .iter()
        .enumerate()
        .find_map(|(r, row)| row
            .iter()
            .enumerate()
            .find_map(|(c, col)| if *col == 'S' { Some((r, c)) } else { None })
        )
        .unwrap();

    let next_frontier = RefCell::new(BTreeSet::from_iter(std::iter::once((start_r, start_c))));
    for _ in 0 .. num_steps {
        let mut curr_frontier = next_frontier.take();
        while let Some((r, c)) = curr_frontier.pop_first() {
            let mut neighbors = vec![];
            if r > 0 {
                neighbors.push((-1, 0));
            }
            if r + 1 < garden.len() {
                neighbors.push((1, 0));
            }
            if c > 0 {
                neighbors.push((0, -1));
            }
            if c + 1 < garden[0].len() {
                neighbors.push((0, 1));
            }

            for (dr, dc) in neighbors {
                let nr = (r as isize + dr) as usize;
                let nc = (c as isize + dc) as usize;
                if garden[nr][nc] != '#' {
                    next_frontier.borrow_mut().insert((nr, nc));
                }
            }
        }
        // println!("{:?}", next_frontier);
    }

    Ok(match part {
        Part::One => next_frontier.borrow().len(),
        
        Part::Two => {
            todo!()
        }
    })
}