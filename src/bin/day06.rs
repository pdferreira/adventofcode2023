use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::parse_sequence;

#[derive(Debug)]
struct Race {
    duration: u64,
    record_distance: u64
}

fn main() {
    println!("example (part1): {:?}", solve("inputs/day06_example", Part::One));
    println!("input (part1): {:?}", solve("inputs/day06", Part::One));
    println!("example (part2): {:?}", solve("inputs/day06_example", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day06", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<u32> {
    let content = fs::read_to_string(path)?;
    
    Ok(match part {
        Part::One =>
            parse_races(&content)?
                .iter()
                .map(|r| r.number_of_ways_to_beat_record())
                .product::<u32>(),
        
        Part::Two => {
            parse_single_race(&content)?.number_of_ways_to_beat_record()
        }
    })
}

fn parse_races(content: &str) -> Result<Vec<Race>> {
    let mut lines_it = content.lines();
    
    let times_str = lines_it.next().ok_or("No line for times")?;
    let times = parse_sequence(&times_str["Time: ".len() ..]);

    let distances_str = lines_it.next().ok_or("No line for distances")?;
    let distances = parse_sequence(&distances_str["Distance: ".len() ..]);

    let races = times
        .iter()
        .zip(distances)
        .map(|(t, d)| Race { duration: *t, record_distance: d })
        .collect();
    
    Ok(races)
}

fn parse_single_race(content: &str) -> Result<Race> {
    let mut lines_it = content.lines();
    
    let time_str = lines_it.next().ok_or("No line for time")?;
    let duration = time_str["Time: ".len() ..].replace(" ", "").parse()?;

    let distance_str = lines_it.next().ok_or("No line for distance")?;
    let record_distance = distance_str["Distance: ".len() ..].replace(" ", "").parse()?;

    Ok(Race { duration, record_distance })
}

impl Race {

    fn number_of_ways_to_beat_record(&self) -> u32 {
        /* Essentially the distance travelled by the boat is given by v * (t - v)
           where v is the velocity AND the time spent pressing the button
           and t is the race duration.

           So what we want to know is how many ways v * (t - v) > d, where d is
           the current record distance. Applying the quadratic roots formula, we get
           that our lowest and highest v is given by

           min_v = (-t + sqrt(t^2 - 4 * d)) / -2
           max_v = (-t - sqrt(t^2 - 4 * d)) / -2
        */
        let t = self.duration as f64;
        let d = self.record_distance as f64;

        let common_sqrt = (t.powf(2.0) - 4.0 * d).sqrt();
        let min_v = (-t + common_sqrt) / -2.0;
        let max_v = (-t - common_sqrt) / -2.0;

        let min_record_v = (min_v + 1.0).floor() as u32; // Round to nearest integer *bigger* than the current min
        let max_record_v = (max_v - 1.0).ceil() as u32; // Round to nearest integer *lower* than the current max
        
        return max_record_v - min_record_v + 1;
    }

}