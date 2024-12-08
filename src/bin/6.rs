use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "./input/6_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    let mut start_r = 0;
    let mut start_c = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == '^' {
                visited.insert((r, c));
                start_r = r;
                start_c = c;
                part1_dfs(&mut visited, r - 1, c, &map, Direction::North);
            }
        }
    }
    let part1_ans = visited.len();
    println!("Part 1: {part1_ans}");

    let mut loop_count = 0;
    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '.' {
                map[r][c] = '#';
                visited.insert((start_r, start_c, Direction::North));
                map[start_r][start_c] = '.';
                if part2_dfs(&mut visited, start_r - 1, start_c, &map, Direction::North) {
                    loop_count += 1;
                }
                map[start_r][start_c] = '^';
                map[r][c] = '.';
                visited.clear();
            }
        }
    }
    println!("Part 2: {loop_count}");
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn part2_dfs(
    visited: &mut HashSet<(usize, usize, Direction)>,
    r: usize,
    c: usize,
    map: &Vec<Vec<char>>,
    direction: Direction,
) -> bool {
    if r >= map.len() || r == usize::MAX || c >= map[0].len() || c == usize::MAX {
        return false;
    }
    if visited.contains(&(r, c, direction.clone())) {
        return true;
    }
    let char = map[r][c];
    visited.insert((r, c, direction.clone()));
    if char == '#' {
        let (r, c) = get_previous_rc(r, c, &direction);
        let direction = get_new_direction(direction);
        return part2_dfs(visited, r, c, map, direction);
    } else if char == '.' {
        let (r, c) = get_next_rc(r, c, &direction);
        return part2_dfs(visited, r, c, map, direction);
    }
    false
}

fn part1_dfs(
    visited: &mut HashSet<(usize, usize)>,
    r: usize,
    c: usize,
    map: &Vec<Vec<char>>,
    direction: Direction,
) {
    if r >= map.len() || r == usize::MAX || c >= map[0].len() || c == usize::MAX {
        return;
    }
    let char = map[r][c];
    if char == '#' {
        let (r, c) = get_previous_rc(r, c, &direction);
        let direction = get_new_direction(direction);
        part1_dfs(visited, r, c, map, direction);
    } else if char == '.' {
        visited.insert((r, c));
        let (r, c) = get_next_rc(r, c, &direction);
        part1_dfs(visited, r, c, map, direction);
    }
}

fn get_next_rc(r: usize, c: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (r.wrapping_sub(1), c),
        Direction::South => (r + 1, c),
        Direction::West => (r, c.wrapping_sub(1)),
        Direction::East => (r, c + 1),
    }
}

fn get_previous_rc(r: usize, c: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (r + 1, c),
        Direction::South => (r.wrapping_sub(1), c),
        Direction::West => (r, c + 1),
        Direction::East => (r, c.wrapping_sub(1)),
    }
}

fn get_new_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}
