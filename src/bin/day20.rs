use std::collections::{HashMap, VecDeque, HashSet};
use adventofcode2023::{Result, Part, run, str::StringOps};

struct NodeConfig<'a> {
    typ: Option<&'a str>,
    dest_names: Vec<&'a str>
}

fn main() {
    run("inputs/day20_example", Part::One, solve);
    run("inputs/day20_example2", Part::One, solve);
    run("inputs/day20", Part::One, solve);

    // run("inputs/day20_example", Part::Two, solve);
    // run("inputs/day20", Part::Two, solve);
}


fn solve(content: &str, part: Part) -> Result<u32> {
    let nodes = parse_modules(content)?;

    let mut enabled_flipflops = HashSet::new();
    let mut conj_inputs = HashMap::new();

    for (name, config) in nodes.iter() {
        for dest in &config.dest_names {
            if nodes.get(dest).filter(|c| c.typ == Some("&")).is_some() {
                conj_inputs
                    .entry(*dest)
                    .or_insert(HashMap::new())
                    .insert(*name, false);
            }
        }
    }

    Ok(match part {
        Part::One => {
            let mut total_high_pulses = 0;
            let mut total_low_pulses = 0;

            for _ in 0 .. 1000 {
                let mut pulse_queue = VecDeque::from_iter(std::iter::once((false, "button", "broadcaster")));
                while let Some((pulse, origin, curr)) = pulse_queue.pop_front() {
                    if pulse {
                        total_high_pulses += 1;
                    } else {
                        total_low_pulses += 1;
                    }
                    // println!("{origin} -{pulse}-> {curr}");

                    if let Some(ns) = nodes.get(curr) {
                        match ns.typ {
                            None => {
                                for dest in &ns.dest_names {
                                    pulse_queue.push_back((pulse, curr, dest));
                                }
                            },
                            Some("%") => {
                                if !pulse {
                                    let new_pulse;
                                    if !enabled_flipflops.remove(curr) {
                                        enabled_flipflops.insert(curr);
                                        new_pulse = true;
                                    } else {
                                        new_pulse = false;
                                    }
                                    
                                    for dest in &ns.dest_names {
                                        pulse_queue.push_back((new_pulse, curr, dest));
                                    }
                                }
                            },
                            Some("&") => {
                                let inputs = conj_inputs.get_mut(curr).unwrap();
                                inputs.insert(origin, pulse);
                                
                                let new_pulse = !inputs.values().all(|v| *v);
                                
                                for dest in &ns.dest_names {
                                    pulse_queue.push_back((new_pulse, curr, dest));
                                }
                            },
                            _ => unreachable!("Unexpected type: {:?}", ns.typ)
                        } 
                    }
                }
                // println!();
            }

            println!("low={total_low_pulses}, high={total_high_pulses}");
            total_low_pulses * total_high_pulses
        },
        
        Part::Two => {
            todo!()
        }
    })
}

fn parse_modules<'a>(content: &str) -> Result<HashMap<&str, NodeConfig>> {
    let mut nodes = HashMap::new();

    let mut lines_it = content.lines();
    while let Some(line) = lines_it.next() {
        let (name_str, dest_str) = line.try_split_once(" -> ")?;
        let name;
        let typ;
        if name_str == "broadcaster" {
            name = name_str;
            typ = None;
        } else {
            name = &name_str[1..];
            typ = Some(&name_str[..1]);
        };

        let dest_names = dest_str.split(", ").collect::<Vec<_>>();

        nodes.insert(name, NodeConfig {
            typ,
            dest_names
        });
    }

    return Ok(nodes);
}