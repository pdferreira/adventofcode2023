use std::fs;

struct Play {
    red_dice_count: u8,
    blue_dice_count: u8,
    green_dice_count: u8
}

struct Counters {
    game_id: u8,
    red: u8,
    blue: u8,
    green: u8
}

struct Game {
    id: u8,
    plays: Vec<Play>
}

fn main() {
    println!("example (part1): {}", solve("inputs/day02_example", /*part_1*/true));
    println!("input (part1): {}", solve("inputs/day02", /*part_1*/true));
    println!("example (part2): {}", solve("inputs/day02_example", /*part_2*/false));
    println!("input (part2): {}", solve("inputs/day02", /*part_2*/false));
}

fn solve(path: &str, part_1: bool) -> u32 {
    let content = fs::read_to_string(path).unwrap();
    let games: Vec<Game> = content
        .lines()
        .map(|l| Game::parse(l))
        .collect();

    let game_counts = games
        .iter()
        .map(|g| Counters {
            game_id: g.id,
            red: g.get_max_count(|p| p.red_dice_count),
            green: g.get_max_count(|p| p.green_dice_count),
            blue: g.get_max_count(|p| p.blue_dice_count)
        });

    if part_1 {
        return game_counts
            .filter(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14)
            .map(|c| u32::from(c.game_id))
            .sum();
    } else {
        return game_counts
            .map(|c| u32::from(c.red) * u32::from(c.green) * u32::from(c.blue))
            .sum();
    }
}

impl Game {
    fn parse(line: &str) -> Game {
        let (game_id_str, plays_str) = line.split_once(": ").unwrap();
        let (_, id_str) = game_id_str.split_once(' ').unwrap();
        let play_strs = plays_str.split("; ");

        return Game {
            id: str::parse(id_str).unwrap(),
            plays: play_strs.map(|p_str| Play::parse(p_str)).collect()
        };
    }

    fn get_max_count(&self, get_count: fn(&Play) -> u8) -> u8 {
        self.plays.iter().map(get_count).max().unwrap()
    }
}

impl Play {
    fn parse(play_str: &str) -> Play {
        let mut play = Play {
            blue_dice_count: 0,
            green_dice_count: 0,
            red_dice_count: 0
        };
        for dice_str in play_str.split(", ") {
            let (count, color) = dice_str.split_once(" ").unwrap();
            match color {
                "blue" => play.blue_dice_count = str::parse(count).unwrap(),
                "red" => play.red_dice_count = str::parse(count).unwrap(),
                "green" => play.green_dice_count = str::parse(count).unwrap(),
                _ => panic!("Unknown color: {color}")
            }
        }

        return play;
    }
}