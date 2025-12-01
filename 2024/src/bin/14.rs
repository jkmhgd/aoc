use std::fs::{self};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn main() {
    let path = "./input/14_input.txt";
    let input = fs::read_to_string(path).unwrap();
    let mut robots = parse(&input);
    part_1(&mut robots);
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let pos_part = parts[0].trim_start_matches("p=");
            let vel_part = parts[1].trim_start_matches("v=");

            let pos_coords: Vec<i32> = pos_part
                .split(',')
                .map(|s| s.parse::<i32>().expect("Failed to parse position"))
                .collect();

            let vel_coords: Vec<i32> = vel_part
                .split(',')
                .map(|s| s.parse::<i32>().expect("Failed to parse velocity"))
                .collect();

            Robot {
                pos: Point {
                    x: pos_coords[0],
                    y: pos_coords[1],
                },
                vel: Point {
                    x: vel_coords[0],
                    y: vel_coords[1],
                },
            }
        })
        .collect()
}

fn part_1(robots: &mut Vec<Robot>) {
    let width = 101;
    let height = 103;
    let mut second_with_tree = (0, i32::MAX);
    for second in 1..=10000 {
        for robot in robots.iter_mut() {
            robot.pos.x = (robot.pos.x + robot.vel.x).rem_euclid(width);
            robot.pos.y = (robot.pos.y + robot.vel.y).rem_euclid(height);
        }
        let orderness = orderness(robots, width, height);
        if second_with_tree.1 > orderness {
            second_with_tree = (second, orderness);
        }
    }
    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;
    for robot in robots.iter() {
        if robot.pos.x < width / 2 && robot.pos.y < height / 2 {
            quadrant_1 += 1;
            continue;
        }
        if robot.pos.x > width / 2 && robot.pos.y < height / 2 {
            quadrant_2 += 1;
            continue;
        }
        if robot.pos.x < width / 2 && robot.pos.y > height / 2 {
            quadrant_3 += 1;
            continue;
        }
        if robot.pos.x > width / 2 && robot.pos.y > height / 2 {
            quadrant_4 += 1;
            continue;
        }
    }
    let part_1 = quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4;
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", second_with_tree.0);
}

fn orderness(robots: &Vec<Robot>, width: i32, height: i32) -> i32 {
    let mut near_edge = 0;
    for robot in robots.iter() {
        if robot.pos.x < width / 20 {
            near_edge += 1;
            continue;
        }
        if robot.pos.x > width - (width as f32 * 0.05) as i32 {
            near_edge += 1;
            continue;
        }
        if robot.pos.y < height / 20 {
            near_edge += 1;
            continue;
        }
        if robot.pos.y > height - (height as f32 * 0.05) as i32 {
            near_edge += 1;
            continue;
        }
    }
    near_edge
}

#[test]
fn day14() {
    let _ = parse(
        "p=0,4 v=3,-3
         p=6,3 v=-1,-3
         p=10,3 v=-1,2
         p=2,0 v=2,-1
         p=0,0 v=1,3
         p=3,0 v=-2,-2
         p=7,6 v=-1,-3
         p=3,0 v=-1,-2
         p=9,3 v=2,3
         p=7,3 v=-1,2
         p=2,4 v=2,-3
         p=9,5 v=-3,-3",
    );
    // assert_eq!(part_1(&mut robots), 12);
    // assert_eq!(part_2(&robots), 0);
}
