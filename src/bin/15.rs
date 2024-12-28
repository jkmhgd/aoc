use std::fs;

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
        let moved = dfs_2(new_r, new_c, &direction, &mut map);
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

fn dfs_2(
    r: usize,
    c: usize,
    // from_char: char,
    direction: &Direction,
    map: &mut Vec<Vec<char>>,
) -> bool {
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
    if map[r][c] == '[' {
        let (new_r, new_c) = get_new_rc(r, c, direction);
        return dfs(new_r, new_c, direction, map);
    }
    if map[r][c] == ']' {
        let (new_r, new_c) = get_new_rc(r, c, direction);
        return dfs(new_r, new_c, direction, map);
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
