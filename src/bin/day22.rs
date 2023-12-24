use kiss3d::light::Light;
use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3};
use kiss3d::window::Window;
use std::fmt::Display;
use std::collections::{HashMap, HashSet};
use adventofcode2023::{Result, Part, run, str::StringOps};

const N: usize = 607;
// green block: ȝ: [(3, 1, 135), (3, 4, 135)]
// gray block: ĸ: [(3, 1, 134), (3, 1, 137)]

// ȝ: [(3, 1, 290), (3, 4, 290)]
// ĸ: [(3, 1, 286), (3, 1, 289)]

// 012345 
//0   
//1   X
//2   ȝ
//3   ȝ
//4   ȝ

#[derive(PartialEq, Hash, Eq)]
struct Point {
    x: u16,
    y: u16,
    z: u16
}

#[derive(PartialEq, Hash, Eq)]
struct Brick {
    label: String,
    bottom_left: Point,
    top_right: Point
}

fn main() {
    // run("inputs/day22_example", Part::One, solve);
    run("inputs/day22", Part::One, solve);

    let b1 = Brick { label: "ĸ".into(), bottom_left: Point { x: 3, y: 1, z: 134 }, top_right: Point { x: 3, y: 1, z: 137 } };
    let b2 = Brick { label: "ȝ".into(), bottom_left: Point { x: 3, y: 1, z: 290 }, top_right: Point { x: 3, y: 4, z: 290 } };
    println!("{}", b2.is_directly_above(&b1));

    let dz = b2.top_right.z - b2.bottom_left.z;
    let new_z = b1.top_right.z + 1;
    println!("{} {}", new_z, dz);

    // run("inputs/day22_example", Part::Two, solve);
    // run("inputs/day22", Part::Two, solve);
}


fn solve(content: &str, part: Part) -> Result<usize> {
    let mut bricks: Vec<Brick> = content
        .lines()
        .enumerate()
        .map(|(i, l)| Brick::parse(i, &l).unwrap())
        .collect();

    bricks.sort_by_key(|b| b.top_right.z);

    let mut brick_dependents = HashMap::new();
    let mut brick_dependencies = HashMap::new();

    for i in 0 .. bricks.len() {
        let dz = bricks[i].top_right.z - bricks[i].bottom_left.z;
        let mut break_after_z = None;

        for j in (0 .. i).rev() {
            if break_after_z.filter(|z| bricks[j].top_right.z < *z).is_some() {
                break;
            }

            //println!("Processing {} with z={} and {:?} for {}", bricks[j].label, bricks[j].top_right.z, break_after_z, bricks[i].label);

            if bricks[i].is_directly_above(&bricks[j]) {
                if break_after_z.is_none() {
                    if bricks[i].label == "ȝ" || bricks[i].label == "ĸ" {
                        print!("{} descending until {}", bricks[i], bricks[j]);
                    }
                    let new_z = bricks[j].top_right.z + 1;
                    bricks[i].bottom_left.z = new_z;
                    bricks[i].top_right.z = new_z + dz;

                    if bricks[i].label == "ȝ" || bricks[i].label == "ĸ" {
                        println!(" becoming {}", bricks[i]);
                    }

                    break_after_z = Some(new_z - 1);
                }

                brick_dependents
                    .entry(j)
                    .or_insert(HashSet::new())
                    .insert(i);

                brick_dependencies
                    .entry(i)
                    .or_insert(HashSet::new())
                    .insert(j);
            }
        }

        if break_after_z.is_none() {
            bricks[i].bottom_left.z = 1;
            bricks[i].top_right.z = 1 + dz;
        }

        // TODO: check if this makes sense
        bricks[..= i].sort_by_key(|b| b.top_right.z);
    }

    for b in &bricks[0 .. (bricks.len() - N)] {
        println!("{}", b);
    }

    // print!("Dependents: {{");
    // for (k, v) in &brick_dependents {
    //     print!("{}: [{}], ", 
    //         bricks[*k].label,
    //         v.iter().map(|j| bricks[*j].label.clone()).collect::<Vec<_>>().join(", "));
    // }
    // println!("}}");

    // print!("Dependencies: {{");
    // for (k, v) in &brick_dependencies {
    //     print!("{}: [{}], ",
    //         bricks[*k].label,
    //         &v.iter().map(|j| bricks[*j].label.clone()).collect::<Vec<_>>().join(", "));
    // }
    // println!("}}");
    
    Ok(match part {
        Part::One => {
            let mut safely_removable_bricks = vec![];
            for i in 0 .. bricks.len() {
                let is_safe = brick_dependents
                    .get(&i)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .all(|j| brick_dependencies
                        .get(j)
                        .unwrap()
                        .len() > 1
                    );
                if is_safe {
                    safely_removable_bricks.push(i);
                }
            }
            println!("{:?}", safely_removable_bricks);
            render_bricks(&bricks, safely_removable_bricks.iter().map(|i| &bricks[*i]).collect::<HashSet<_>>());
            safely_removable_bricks.len()
        },
        
        Part::Two => {
            todo!()
        }
    })
}

fn render_bricks(bricks: &Vec<Brick>, highlights: HashSet<&Brick>) {
    let mut window = Window::new("Map");
    window.set_background_color(1.0, 1.0, 1.0);
    window.scene_mut().append_rotation(&UnitQuaternion::new(Vector3::z() * -std::f32::consts::FRAC_PI_2));
    window.scene_mut().append_rotation(&UnitQuaternion::new(Vector3::x() * -std::f32::consts::FRAC_PI_2));
    window.scene_mut().append_translation(&Translation3::new(0.0, -2.0, 2.0));
    
    let mut cubes = vec![];
    for i in 0 .. (bricks.len() - N) {
        let b = &bricks[i];
        let x_len = (b.top_right.x - b.bottom_left.x + 1) as f32;
        let y_len = (b.top_right.y - b.bottom_left.y + 1) as f32;
        let z_len = (b.top_right.z - b.bottom_left.z + 1) as f32;
        let mut c = window.add_cube(
            x_len,
            y_len,
            z_len
        );
        c.append_translation(&Translation3::new(x_len / 2.0, y_len / 2.0, z_len / 2.0));
        
        c.append_translation(&Translation3::new(
            b.bottom_left.x as f32,
            b.bottom_left.y as f32,
            b.bottom_left.z as f32
        ));

        if highlights.contains(&b) {
            c.set_color(0.5, 0.5, 0.5);
        } else if i % 3 == 0 {
            c.set_color((0.1 + i as f32) / (0.1 + bricks.len() as f32), (i % 100) as f32 / 100.0, 0.0);
        } else if i % 3 == 1 {
            c.set_color(0.0, (0.1 + i as f32) / (0.1 + bricks.len() as f32), (i % 100) as f32 / 100.0);
        } else {
            c.set_color((i % 100) as f32 / 100.0, 0.0, (0.1 + i as f32) / (0.1 + bricks.len() as f32));
        }

        cubes.push(c);
    }
    
    window.set_light(Light::StickToCamera);
    while window.render() { }
}

impl Brick {

    fn parse(i: usize, line: &str) -> Result<Brick> {
        let (p1_str, p2_str) = line.try_split_once("~")?;
        Ok(Brick {
            label: char::from_u32('A' as u32 + i as u32).unwrap().to_string(),
            bottom_left: Point::parse(&p1_str),
            top_right: Point::parse(&p2_str)
        })
    }

    fn is_directly_above(&self, other: &Brick) -> bool {
        self.bottom_left.z > other.top_right.z
        && self.bottom_left.x <= other.top_right.x
        && self.top_right.x >= other.bottom_left.x
        && self.bottom_left.y <= other.top_right.y
        && self.top_right.y >= other.bottom_left.y
    }

}

impl Display for Brick {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: [{}, {}]", self.label, self.bottom_left, self.top_right))
    }

}

impl Point {

    fn parse(s: &str) -> Point {
        let mut coords_it = s.splitn(3, ',').map(|cs| cs.parse().unwrap());
        Point {
            x: coords_it.next().unwrap(),
            y: coords_it.next().unwrap(),
            z: coords_it.next().unwrap()
        }
    }

}

impl Display for Point {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }

}