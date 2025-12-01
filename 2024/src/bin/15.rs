use std::{collections::HashSet, fs};

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let path = "./input/15_input.txt";
    let input = fs::read_to_string(path).unwrap();
    part_1(&input);
    part_2(&input);
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map = vec![];
    let mut moves = vec![];
    let mut done_parsing_map = false;

    for line in input.lines() {
        if line == "" {
            done_parsing_map = true;
            continue;
        }
        if !done_parsing_map {
            map.push(line.chars().collect());
        } else {
            for char in line.chars() {
                moves.push(char);
            }
        }
    }
    (map, moves)
}

fn part_1(input: &str) {
    let (mut map, moves) = parse(&input);
    let mut robot_r: usize = 0;
    let mut robot_c: usize = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == '@' {
                robot_r = r;
                robot_c = c;
                break;
            }
        }
    }

    for mov in moves {
        let direction = get_direction(mov);
        let (new_r, new_c) = get_new_rc(robot_r, robot_c, &direction);
        let moved = dfs(new_r, new_c, &direction, &mut map);
        if moved {
            map[robot_r][robot_c] = '.';
            robot_r = new_r;
            robot_c = new_c;
            map[robot_r][robot_c] = '@';
        }
    }

    let mut total = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == 'O' {
                total += (100 * r) + c;
            }
        }
    }
    println!("{}", total);
}

fn dfs(r: usize, c: usize, direction: &Direction, map: &mut Vec<Vec<char>>) -> bool {
    if r == usize::MAX || r >= map.len() || c == usize::MAX || c >= map[0].len() {
        println!("ts");
        return false;
    }
    if map[r][c] == '#' {
        return false;
    }
    if map[r][c] == '.' {
        map[r][c] = 'O';
        return true;
    }
    if map[r][c] == 'O' {
        let (new_r, new_c) = get_new_rc(r, c, direction);
        return dfs(new_r, new_c, direction, map);
    }
    panic!("not possible");
}

fn part_2(input: &str) {
    let (mut map, moves) = parse(&input);
    for r in 0..map.len() {
        let mut c = 0;
        while c < map[0].len() {
            let char = map[r][c];
            if char == '#' {
                map[r].insert(c + 1, '#');
            }
            if char == 'O' {
                map[r][c] = '[';
                map[r].insert(c + 1, ']');
            }
            if char == '.' {
                map[r].insert(c + 1, '.');
            }
            if char == '@' {
                map[r].insert(c + 1, '.');
            }
            c += 2;
        }
    }

    let mut robot_r: usize = 0;
    let mut robot_c: usize = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == '@' {
                robot_r = r;
                robot_c = c;
                break;
            }
        }
    }

    for mov in moves {
        let direction = get_direction(mov);
        let (new_r, new_c) = get_new_rc(robot_r, robot_c, &direction);
        let mut boxes_to_move = HashSet::new();
        let moved;
        if direction == Direction::North || direction == Direction::South {
            let char = map[new_r][new_c];
            if char == '[' {
                moved = dfs_2(new_r, new_c, &direction, &mut map, &mut boxes_to_move) && dfs_2(new_r, new_c + 1, &direction, &mut map, &mut boxes_to_move);
            } else if char == ']' {
                moved = dfs_2(new_r, new_c - 1, &direction, &mut map, &mut boxes_to_move) && dfs_2(new_r, new_c, &direction, &mut map, &mut boxes_to_move);
            } else {
                moved = dfs_2(new_r, new_c, &direction, &mut map, &mut boxes_to_move);
            }

        } else {
            moved = dfs_2(new_r, new_c, &direction, &mut map, &mut boxes_to_move);
        }
        if moved {
            let mut vec: Vec<(usize, usize)> = boxes_to_move.into_iter().collect();
            if direction == Direction::North {
                vec.sort_by(|a, b| a.0.cmp(&b.0));

            } else if direction == Direction::South {
                vec.sort_by(|a, b| b.0.cmp(&a.0));
            } else if direction == Direction::West {
                vec.sort_by(|a, b| a.1.cmp(&b.1));
            } else if direction == Direction::East {
                vec.sort_by(|a, b| b.1.cmp(&a.1));
            }

            for (r, c) in vec {
                let (new_r, new_c) = get_new_rc(r, c, &direction);
                map[new_r][new_c] = map[r][c];
                map[r][c] = '.';
            }
            map[robot_r][robot_c] = '.';
            robot_r = new_r;
            robot_c = new_c;
            map[robot_r][robot_c] = '@';
        }
    }

    let mut total = 0;
    for (r, row) in map.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            if *char == '[' {
                total += (100 * r) + c;
            }
        }
    }
    println!("{}", total);
}

fn dfs_2(
    r: usize,
    c: usize,
    direction: &Direction,
    map: &mut Vec<Vec<char>>,
    boxes_to_move: &mut HashSet<(usize, usize)>,
) -> bool {
    if r == usize::MAX || r >= map.len() || c == usize::MAX || c >= map[0].len() {
        return false;
    }
    if map[r][c] == '#' {
        return false;
    }
    if map[r][c] == '.' {
        return true;
    }
    if *direction == Direction::North || *direction == Direction::South {
        let (new_r, new_c) = get_new_rc(r, c, direction);
        let next_char = map[new_r][new_c];
        if next_char == '[' {
            if map[r][c] == '[' {
                boxes_to_move.insert((r, c));
                return dfs_2(new_r, new_c, &direction, map, boxes_to_move) && dfs_2(new_r, new_c + 1, &direction, map, boxes_to_move);
            }
            if map[r][c] == ']' {
                boxes_to_move.insert((r, c));
                return dfs_2(new_r, new_c - 1, &direction, map, boxes_to_move) && dfs_2(new_r, new_c, &direction, map, boxes_to_move) && dfs_2(new_r, new_c + 1, &direction, map, boxes_to_move);
            }
        } else if next_char == ']' {
            if map[r][c] == '[' {
                boxes_to_move.insert((r, c));
                return dfs_2(new_r, new_c - 1, &direction, map, boxes_to_move) && dfs_2(new_r, new_c, &direction, map, boxes_to_move) && dfs_2(new_r, new_c + 1, &direction, map, boxes_to_move);
            }
            if map[r][c] == ']' {
                boxes_to_move.insert((r, c));
                return dfs_2(new_r, new_c - 1, &direction, map, boxes_to_move) && dfs_2(new_r, new_c, &direction, map, boxes_to_move);
            }
        } else {
            if map[r][c] == '[' {
                boxes_to_move.insert((r, c));
                let (new_r, new_c) = get_new_rc(r, c, direction);
                return dfs_2(new_r, new_c, direction, map, boxes_to_move) && dfs_2(new_r, new_c + 1, direction, map, boxes_to_move);
            }
            if map[r][c] == ']' {
                boxes_to_move.insert((r, c));
                let (new_r, new_c) = get_new_rc(r, c, direction);
                return dfs_2(new_r, new_c - 1, direction, map, boxes_to_move) && dfs_2(new_r, new_c, direction, map, boxes_to_move);
            }
        }

    } else {
        if map[r][c] == '[' || map[r][c] == ']' {
            boxes_to_move.insert((r, c));
            let (new_r, new_c) = get_new_rc(r, c, direction);
            return dfs_2(new_r, new_c, direction, map, boxes_to_move);
        }
    }
    panic!("not possible");
}

fn get_new_rc(r: usize, c: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (r.wrapping_sub(1), c),
        Direction::South => (r + 1, c),
        Direction::West => (r, c.wrapping_sub(1)),
        Direction::East => (r, c + 1),
    }
}

fn get_direction(ch: char) -> Direction {
    match ch {
        '^' => Direction::North,
        'v' => Direction::South,
        '<' => Direction::West,
        '>' => Direction::East,
        _ => panic!("Invalid move."),
    }
}
