use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use adventofcode2023::{Result, Part, run, str::StringOps};

lazy_static! {
    static ref COND_REGEX: Regex = Regex::new(r"(?P<field>\w+)(?P<op><|>)(?P<value>\d+)").unwrap();
    static ref PART_REGEX: Regex = Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}").unwrap();
}

type WorkflowName = String;

#[derive(Clone, Copy)]
struct MachinePart {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

struct Workflows {
    nodes: HashMap<WorkflowName, Box<dyn Fn(&MachinePart) -> WorkflowName>>
}

fn main() {
    run("inputs/day19_example", Part::One, solve);
    run("inputs/day19", Part::One, solve);

    // run("inputs/day19_example", Part::Two, solve);
    // run("inputs/day19", Part::Two, solve);
}


fn solve(content: &str, part: Part) -> Result<u32> {
    let lines_it = &mut content.lines();

    Ok(match part {
        Part::One => {
            let workflows = Workflows::parse(lines_it)?;

            let mut machine_parts = vec![]; 
            while let Some(line) = lines_it.next() {
                machine_parts.push(MachinePart::parse(line)?);
            }

            machine_parts
                .into_iter()
                .filter(|mp| workflows.is_accepted(mp))
                .map(|mp| mp.value())
                .sum()
        },
        
        Part::Two => {
            todo!()
        }
    })
}

impl Workflows {

    fn parse<'a>(lines_it: &mut impl Iterator<Item = &'a str>) -> Result<Workflows> {
        let mut nodes = HashMap::new();
        while let Some(line) = lines_it.next().filter(|l| !l.is_empty()) {
            let (name, flow_str) = line.try_split_once("{")?;
            let mut edges_str = flow_str.trim_end_matches('}').split(',').rev();
    
            let default_case = edges_str.next().unwrap().to_string();
            let mut next_state_fn: Box<dyn Fn(&MachinePart) -> WorkflowName> = Box::new(move |_| default_case.clone());
            for e_str in edges_str {
                let (cond_str, next_state_str) = e_str.try_split_once(":")?;
                let cond_captures = COND_REGEX.captures(cond_str).unwrap();
    
                let field = String::from(&cond_captures["field"]);
                let op = if &cond_captures["op"] == ">" {
                    std::cmp::PartialOrd::gt
                } else {
                    std::cmp::PartialOrd::lt
                };
                let value = cond_captures["value"].parse()?;
                let next_state = String::from(next_state_str);
    
                next_state_fn = Box::new(move |mp: &MachinePart| {
                    if op(&mp.get(&field), &value) {
                        next_state.clone()
                    } else {
                        next_state_fn(mp)
                    }
                });
            }
            nodes.insert(name.to_string(), next_state_fn);
        }

        Ok(Workflows { nodes })    
    }

    fn is_accepted(&self, mp: &MachinePart) -> bool {
        let mut curr_state = String::from("in");
        while curr_state != "A" && curr_state != "R" {
            let next_state_fn = self.nodes.get(curr_state.as_str()).unwrap();
            curr_state = next_state_fn(mp);
        }
        // println!("{}", curr_state);
        return curr_state == "A";
    }

}

impl MachinePart {

    fn parse(line: &str) -> Result<MachinePart> {
        let part_captures = PART_REGEX.captures(line).unwrap();
        Ok(MachinePart {
            x: part_captures["x"].parse()?,
            m: part_captures["m"].parse()?,
            a: part_captures["a"].parse()?,
            s: part_captures["s"].parse()?
        })
    }

    fn get(&self, field_name: &str) -> u32 {
        match field_name {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!()
        }
    }

    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

}