use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path = "./input/10_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (r, row) in map.iter().enumerate() {
        for (c, height) in row.iter().enumerate() {
            if *height == '0' {
                dfs(r + 1, c, *height, &map, &mut sum_part_1, &mut visited, true);
                dfs(r.wrapping_sub(1), c, *height, &map, &mut sum_part_1, &mut visited, true);
                dfs(r, c + 1, *height, &map, &mut sum_part_1, &mut visited, true);
                dfs(r, c.wrapping_sub(1), *height, &map, &mut sum_part_1, &mut visited, true);
                visited.clear();

                dfs(r + 1, c, *height, &map, &mut sum_part_2, &mut visited, false);
                dfs(r.wrapping_sub(1), c, *height, &map, &mut sum_part_2, &mut visited, false);
                dfs(r, c + 1, *height, &map, &mut sum_part_2, &mut visited, false);
                dfs(r, c.wrapping_sub(1), *height, &map, &mut sum_part_2, &mut visited, false);
                visited.clear();
            }
        }
    }

    println!("Part 1: {sum_part_1}");
    println!("Part 2: {sum_part_2}");
}

fn dfs(
    r: usize,
    c: usize,
    prev_height: char,
    map: &Vec<Vec<char>>,
    sum: &mut u32,
    visited: &mut HashSet<(usize, usize)>,
    check_visited: bool,
) {
    if r == usize::MAX || r >= map.len() || c == usize::MAX || c >= map[0].len() {
        return;
    }
    if check_visited && visited.contains(&(r, c)) {
        return;
    }
    let current = map[r][c];
    let prev_height = prev_height.to_digit(10).unwrap();
    let current_height = current.to_digit(10).unwrap();
    if prev_height + 1 != current_height {
        return;
    }
    if current == '9' {
        visited.insert((r, c));
        *sum += 1;
        return;
    }
    dfs(r + 1, c, current, map, sum, visited, check_visited);
    dfs(r.wrapping_sub(1), c, current, map, sum, visited, check_visited);
    dfs(r, c + 1, current, map, sum, visited, check_visited);
    dfs(r, c.wrapping_sub(1), current, map, sum, visited, check_visited);
}
