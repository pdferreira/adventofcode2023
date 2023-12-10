use std::{fs, collections::LinkedList};
use adventofcode2023::{Result, Part};

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
enum Tile {
    NorthAndSouth = b'|',
    NorthAndEast = b'L',
    NorthAndWest = b'J',
    WestAndEast = b'-',
    SouthAndWest = b'7',
    SouthAndEast = b'F',
    #[allow(dead_code)]
    Ground = b'.',
    Start = b'S',
}

struct PipeMap {
    tiles: Vec<Vec<Tile>>
}

#[derive(PartialEq, Clone, Copy)]
struct Position {
    r: usize,
    c: usize
}

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
enum TileState {
    Unknown = b'?',
    Inside = b'I',
    Outside = b'O',
    Boundary = b'B',
}

fn main() {
    println!("example1 (part1): {:?}", solve("inputs/day10_example1", Part::One));
    println!("example2 (part1): {:?}", solve("inputs/day10_example2", Part::One));
    println!("input (part1): {:?}", solve("inputs/day10", Part::One));
    println!("example3 (part2): {:?}", solve("inputs/day10_example3", Part::Two));
    println!("example4 (part2): {:?}", solve("inputs/day10_example4", Part::Two));
    println!("example5 (part2): {:?}", solve("inputs/day10_example5", Part::Two));
    println!("example6 (part2): {:?}", solve("inputs/day10_example6", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day10", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<usize> {
    let content = fs::read_to_string(path)?;
    let map = PipeMap::parse(&content);
    let pipe_loop = map.find_loop().ok_or("No start or loop found")?;

    Ok(match part {
        Part::One =>
            pipe_loop.len() / 2,
        
        Part::Two => { 
            map.find_area_within_loop(pipe_loop.into_iter())
        }
    })
}

impl PipeMap {

    fn parse(content: &str) -> PipeMap {
        let tiles = content
            .lines()
            .map(|l| l
                .chars()
                .map(|c| unsafe { std::mem::transmute(c as u8) })
                .collect()
            )
            .collect();

        PipeMap { tiles }
    }

    fn find_start(&self) -> Option<Position> {
        for r in 0 .. self.tiles.len() {
            let row = &self.tiles[r];
            for c in 0 .. row.len() {
                if row[c] == Tile::Start {
                    return Some(Position { r, c })
                }
            }
        }

        return None
    }

    fn infer_tile(&self, pos: Position) -> Tile {
        let mut connections = self.find_connected_tiles(pos);
        connections.sort_by_key(|c| c.r);

        if let [c1, c2] = connections.as_slice() {
            if c1.r == c2.r {
                Tile::WestAndEast
            } else if c1.c == c2.c {
                Tile::NorthAndSouth
            } else if c1.r + 1 == c2.r {
                if pos.r == c2.r {
                    if pos.c < c2.c {
                        Tile::NorthAndEast
                    } else {
                        Tile::NorthAndWest
                    }
                } else if pos.c < c1.c {
                    Tile::SouthAndEast
                } else {
                    Tile::SouthAndWest
                }
            } else {
                todo!("Start at {:?} with connections {:?}", pos, connections)
            }
        } else {
            panic!()
        }
    }

    fn find_connected_tiles(&self, pos: Position) -> Vec<Position> {
        let Position { r, c } = pos; 
        let mut connected = vec![];
        
        if r > 0 && [Tile::SouthAndEast, Tile::NorthAndSouth, Tile::SouthAndWest].contains(&self.tiles[r - 1][c]) {
            connected.push(Position { r: r - 1, c });
        }

        if c > 0 && [Tile::NorthAndEast, Tile::SouthAndEast, Tile::WestAndEast].contains(&self.tiles[r][c - 1]) {
            connected.push(Position { r, c: c - 1 })
        }

        if r + 1 < self.tiles.len() && [Tile::NorthAndEast, Tile::NorthAndSouth, Tile::NorthAndWest].contains(&self.tiles[r + 1][c]) {
            connected.push(Position { r: r + 1, c: c })
        }

        if c + 1 < self.tiles[r].len() && [Tile::WestAndEast, Tile::SouthAndWest, Tile::NorthAndWest].contains(&self.tiles[r][c + 1]) {
            connected.push(Position { r: r, c: c + 1 })
        }

        return connected;
    }

    fn next(&self, prev: Position, curr: Position) -> Option<Position> {
        let Position { r, c } = curr;
        match self.tiles[r][c] {
            Tile::Ground => None,
            Tile::Start => panic!("Unexpected start tile"),
            Tile::NorthAndEast => Some(if prev.r + 1 == r { 
                Position { r, c: c + 1 } 
            } else {
                Position { r: r - 1, c }
            }),
            Tile::NorthAndSouth => Some(if prev.r + 1 == r {
                Position { r: r + 1, c }
            } else {
                Position { r: r - 1, c }
            }),
            Tile::NorthAndWest => Some(if prev.r + 1 == r {
                Position { r, c: c - 1 }
            } else {
                Position { r: r - 1, c }
            }),
            Tile::SouthAndEast => Some(if prev.r == r + 1 {
                Position { r, c: c + 1 }
            } else {
                Position { r: r + 1, c }
            }),
            Tile::SouthAndWest => Some(if prev.r == r + 1 {
                Position { r, c: c - 1 }
            } else {
                Position { r: r + 1, c }
            }),
            Tile::WestAndEast => Some(if prev.c + 1 == c {
                Position { r, c: c + 1 }
            } else {
                Position { r, c: c - 1 }
            })
        }
    }

    fn find_loop(&self) -> Option<LinkedList<Position>> {
        let start_pos = self.find_start()?;
        let connections = self.find_connected_tiles(start_pos);

        let mut pipe_loop = LinkedList::from([start_pos]); 
        let mut curr_frontier: Vec<_> = connections.iter().map(|p| (start_pos, *p)).collect();
        loop {
            // println!("[{}] {:?}", distance, curr_frontier);
            curr_frontier.dedup_by(|(_, p1), (_, p2)| p1 == p2);
            pipe_loop.push_back(curr_frontier[0].1);
            
            if curr_frontier.len() < 2 {
                break;
            }
            assert_eq!(curr_frontier.len(), 2);
            pipe_loop.push_front(curr_frontier[1].1);

            for (prev, curr) in curr_frontier.as_mut_slice() {
                let new_curr = self.next(*prev, *curr)?;
                *prev = *curr;
                *curr = new_curr;
            }
        }

        return Some(pipe_loop)
    }

    fn find_area_within_loop(&self, pipe_loop: impl Iterator<Item = Position>) -> usize {
        // Initialize a matrix with all states set to Unknown except for the Boundaries
        let mut states = vec![vec![TileState::Unknown; self.tiles[0].len()]; self.tiles.len()];
        for Position { r, c } in pipe_loop {
            states[r][c] = TileState::Boundary;
        }

        // Initialize a slightly bigger matrix to store if the corners of `states` are outside or inside.
        // We know the outer corners all around are necessarily outside, so those get marked right away.
        let outside_len = self.tiles.len() + 1;
        let outside_row_len = self.tiles[0].len() + 1;

        let mut sample_outside = vec![None; outside_row_len];
        sample_outside[0] = Some(true);
        sample_outside[outside_row_len - 1] = Some(true);
        
        let mut outside = vec![sample_outside; outside_len];
        outside[0] = vec![Some(true); outside_row_len];
        outside[outside_len - 1] = vec![Some(true); outside_row_len];

        // For each map position, keep updating the southwest and southeast corners depending on the shape
        // of the boundary. At the same time, use the northeast corner info to understand if Unknown positions
        // should be marked as outside or inside.
        for r in 0 .. states.len() {
            let t_row = &self.tiles[r];

            if states[r][0] != TileState::Boundary {
                outside[r + 1][0] = Some(true);
                outside[r + 1][1] = Some(true);
            }

            for c in 0 .. states[r].len() {
                let is_northwest_outside = outside[r][c];

                if states[r][c] == TileState::Boundary {
                    let tile_to_match = if t_row[c] == Tile::Start {
                        self.infer_tile(Position { r, c })
                    } else {
                        t_row[c]
                    };
                    match tile_to_match {
                        Tile::NorthAndEast => {
                            outside[r + 1][c] = is_northwest_outside;
                            outside[r + 1][c + 1] = is_northwest_outside;
                        },
                        Tile::NorthAndSouth => {
                            outside[r + 1][c] = is_northwest_outside;
                            outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                        },
                        Tile::NorthAndWest => {
                            outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                            outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                        },
                        Tile::WestAndEast => {
                            outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                            outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                        },
                        Tile::SouthAndEast => {
                            outside[r + 1][c] = is_northwest_outside;
                            outside[r + 1][c + 1] = is_northwest_outside.map(|o| !o);
                        },
                        Tile::SouthAndWest => {
                            outside[r + 1][c] = is_northwest_outside.map(|o| !o);
                            outside[r + 1][c + 1] = is_northwest_outside;
                        },
                        Tile::Ground => {
                            outside[r + 1][c] = is_northwest_outside;
                            outside[r + 1][c + 1] = is_northwest_outside;
                        },
                        _ => panic!("Not expecting to find Start tile at all")
                    }
                } else if states[r][c] == TileState::Unknown {
                    states[r][c] = if outside[r][c].unwrap() {
                        TileState::Outside
                    } else {
                        TileState::Inside
                    };
                    outside[r + 1][c] = outside[r][c];
                    outside[r + 1][c + 1] = outside[r][c];
                }
            }
        }

        // println!();
        // for os in outside {
        //     let line = String::from_iter(os.iter().map(|o| match o {
        //         None => '?',
        //         Some(true) => 'T',
        //         Some(false) => 'F', 
        //     }));
        //     println!("{}", line);
        // }

        // println!();
        // for (ts, ss) in self.tiles.iter().zip(&states) {
        //     let line = String::from_iter(
        //         ts
        //             .iter()
        //             .zip(ss)
        //             .map(|(t, s)| if *s == TileState::Boundary { t.to_char() } else { s.to_char() })
        //         );
        //     println!("{}", line);
        // }

        states
            .iter()
            .map(|ss| ss
                .iter()
                .filter(|s| **s == TileState::Inside)
                .count()
            )
            .sum()
    }

}

impl core::fmt::Debug for PipeMap {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ts in &self.tiles {
            let line = String::from_iter(ts.iter().map(|t| t.to_char()));
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl core::fmt::Debug for Position {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}

impl core::fmt::Debug for Tile {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }

}

impl Tile {

    fn to_char(&self) -> char {
        char::from(*self as u8)
    }

}

impl TileState {

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        char::from(*self as u8)
    }

}
