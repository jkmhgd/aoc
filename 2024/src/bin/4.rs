use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "./input/4_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut char_array: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        char_array.push(chars);
    }

    part_1(&char_array);
    part_2(&char_array);
}

enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

fn part_1(char_array: &Vec<Vec<char>>) {
    let len = char_array.len();
    let mut count = 0;

    for (r, row) in char_array.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let can_n = r >= 3;
            let can_s = r <= len - 4;
            let can_w = c >= 3;
            let can_e = c <= len - 4;
            let can_nw = can_n && can_w;
            let can_ne = can_n && can_e;
            let can_sw = can_s && can_w;
            let can_se = can_s && can_e;

            if can_n && check_for_xmas(&char_array, r, c, Direction::North) {
                count += 1;
            }
            if can_s && check_for_xmas(&char_array, r, c, Direction::South) {
                count += 1;
            }
            if can_w && check_for_xmas(&char_array, r, c, Direction::West) {
                count += 1;
            }
            if can_e && check_for_xmas(&char_array, r, c, Direction::East) {
                count += 1;
            }
            if can_nw && check_for_xmas(&char_array, r, c, Direction::NorthWest) {
                count += 1;
            }
            if can_ne && check_for_xmas(&char_array, r, c, Direction::NorthEast) {
                count += 1;
            }
            if can_sw && check_for_xmas(&char_array, r, c, Direction::SouthWest) {
                count += 1;
            }
            if can_se && check_for_xmas(&char_array, r, c, Direction::SouthEast) {
                count += 1;
            }
        }
    }
    println!("Part 1: {count}");
}

fn check_for_xmas(char_array: &Vec<Vec<char>>, r: usize, c: usize, direction: Direction) -> bool {
    match direction {
        Direction::North => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r - 1][c] == 'M';
            let l3 = char_array[r - 2][c] == 'A';
            let l4 = char_array[r - 3][c] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::South => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r + 1][c] == 'M';
            let l3 = char_array[r + 2][c] == 'A';
            let l4 = char_array[r + 3][c] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::West => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r][c - 1] == 'M';
            let l3 = char_array[r][c - 2] == 'A';
            let l4 = char_array[r][c - 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::East => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r][c + 1] == 'M';
            let l3 = char_array[r][c + 2] == 'A';
            let l4 = char_array[r][c + 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::NorthWest => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r - 1][c - 1] == 'M';
            let l3 = char_array[r - 2][c - 2] == 'A';
            let l4 = char_array[r - 3][c - 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::NorthEast => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r - 1][c + 1] == 'M';
            let l3 = char_array[r - 2][c + 2] == 'A';
            let l4 = char_array[r - 3][c + 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::SouthWest => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r + 1][c - 1] == 'M';
            let l3 = char_array[r + 2][c - 2] == 'A';
            let l4 = char_array[r + 3][c - 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
        Direction::SouthEast => {
            let l1 = char_array[r][c] == 'X';
            let l2 = char_array[r + 1][c + 1] == 'M';
            let l3 = char_array[r + 2][c + 2] == 'A';
            let l4 = char_array[r + 3][c + 3] == 'S';
            return l1 && l2 && l3 && l4;
        }
    }
}

fn part_2(char_array: &Vec<Vec<char>>) {
    let len = char_array.len();
    let mut mas_count = 0;

    for (r, row) in char_array.iter().enumerate() {
        for (c, char) in row.iter().enumerate() {
            let can_n = r >= 1 && r <= len - 2;
            let can_s = can_n;
            let can_w = c >= 1 && c <= len - 2;
            let can_e = can_w;
            let can_nw = can_n && can_w;
            let can_ne = can_n && can_e;
            let can_sw = can_s && can_w;
            let can_se = can_s && can_e;

            if *char == 'A' {
                let nw = can_nw && check_for_mas(&char_array, r, c, Direction::NorthWest);
                let ne = can_ne && check_for_mas(&char_array, r, c, Direction::NorthEast);
                let sw = can_sw && check_for_mas(&char_array, r, c, Direction::SouthWest);
                let se = can_se && check_for_mas(&char_array, r, c, Direction::SouthEast);

                if nw && ne {
                    mas_count += 1;
                    continue;
                }
                if nw && sw {
                    mas_count += 1;
                    continue;
                }
                if ne && se {
                    mas_count += 1;
                    continue;
                }
                if sw && se {
                    mas_count += 1;
                    continue;
                }
            }
        }
    }
    println!("Part 2: {mas_count}");
}

fn check_for_mas(char_array: &Vec<Vec<char>>, r: usize, c: usize, direction: Direction) -> bool {
    match direction {
        Direction::North => {
            let l1 = char_array[r + 1][c] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r - 1][c] == 'S';
            return l1 && l2 && l3;
        }
        Direction::South => {
            let l1 = char_array[r - 1][c] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r + 1][c] == 'S';
            return l1 && l2 && l3;
        }
        Direction::West => {
            let l1 = char_array[r][c + 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r][c - 1] == 'S';
            return l1 && l2 && l3;
        }
        Direction::East => {
            let l1 = char_array[r][c - 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r][c + 1] == 'S';
            return l1 && l2 && l3;
        }
        Direction::NorthWest => {
            let l1 = char_array[r + 1][c + 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r - 1][c - 1] == 'S';
            return l1 && l2 && l3;
        }
        Direction::NorthEast => {
            let l1 = char_array[r + 1][c - 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r - 1][c + 1] == 'S';
            return l1 && l2 && l3;
        }
        Direction::SouthWest => {
            let l1 = char_array[r - 1][c + 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r + 1][c - 1] == 'S';
            return l1 && l2 && l3;
        }
        Direction::SouthEast => {
            let l1 = char_array[r - 1][c - 1] == 'M';
            let l2 = char_array[r][c] == 'A';
            let l3 = char_array[r + 1][c + 1] == 'S';
            return l1 && l2 && l3;
        }
    }
}
