use std::{cmp, collections::HashSet, fs::File, io::{BufRead, BufReader}};

fn main() {
    let file_path = "./input/12_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut plots: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let chars: Vec<char> = line.chars().collect();
        plots.push(chars);
    }

    let mut part_1_total_price = 0;
    let mut part_2_total_price = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for (r, row) in plots.iter().enumerate() {
        for (c, plant) in row.iter().enumerate() {
            if visited.contains(&(r, c)) {
                continue;
            }

            let mut area: HashSet<(usize, usize)> = HashSet::new();
            let mut perimeter: HashSet<(usize, usize, usize, usize)> = HashSet::new();
            area.insert((r, c));
            dfs(r + 1, c, r, c, *plant, &plots, &mut area, &mut perimeter);
            dfs(r.wrapping_sub(1), c, r, c, *plant, &plots, &mut area, &mut perimeter);
            dfs(r, c + 1, r, c, *plant, &plots, &mut area, &mut perimeter);
            dfs(r, c.wrapping_sub(1), r, c, *plant, &plots, &mut area, &mut perimeter);
            part_1_total_price += area.len() * perimeter.len();
            part_2_total_price += area.len() * count_sides(perimeter);
            
            visited.extend(area);
        }
    }

    println!("Part 1: {}", part_1_total_price);
    println!("Part 2: {}", part_2_total_price);
}

fn dfs(
    r: usize,
    c: usize,
    from_r: usize,
    from_c: usize,
    plant: char,
    plots: &Vec<Vec<char>>,
    area: &mut HashSet<(usize, usize)>,
    perimeter: &mut HashSet<(usize, usize, usize, usize)>,
) {
    if area.contains(&(r, c)) {
        return;
    }
    if r == usize::MAX || r >= plots.len() || c == usize::MAX || c >= plots[0].len() {
        perimeter.insert((from_r, from_c, r, c));
        return;
    }
    let current = plots[r][c];
    if current != plant {
        perimeter.insert((from_r, from_c, r, c));
        return;
    }
    area.insert((r, c));
    dfs(r + 1, c, r, c, plant, plots, area, perimeter);
    dfs(r.wrapping_sub(1), c, r, c, plant, plots, area, perimeter);
    dfs(r, c + 1, r, c, plant, plots, area, perimeter);
    dfs(r, c.wrapping_sub(1), r, c, plant, plots, area, perimeter);
}

fn count_sides(perimeter: HashSet<(usize, usize, usize, usize)>) -> usize {
    let mut sides: HashSet<(bool, usize, usize, usize, usize)> = HashSet::new();
    // is_col_move from_r/c to_r/c from_c/r to_c/r
    for (from_r, from_c, r, c) in perimeter.iter() {
        let mut is_col_move = false;
        
        if from_r == r && from_c != c {
            is_col_move = true;
            let mut to_swap: Vec<Option<(bool, usize, usize, usize, usize)>> = vec![];
            let mut new: Option<(bool, usize, usize, usize, usize)> = None;
            let mut found = false; 

            for side in sides.iter() {
                let is_new_start = side.3.wrapping_sub(1) == *r;
                let is_new_end = side.4 + 1 == *r;
                if side.0 == is_col_move && side.1 == *from_c && side.2 == *c && (is_new_start || is_new_end) {
                    if !found {
                        found = true;
                        to_swap.push(Some(*side));
                        if is_new_start {
                            new = Some((side.0, side.1, side.2, *r, side.4));
                        } else {
                            new = Some((side.0, side.1, side.2, side.3, *r));
                        }
                    } else {
                        to_swap.push(Some(*side));
                        let mut temp = new.unwrap();
                        if is_new_start {
                            temp.3 = cmp::min(temp.3, *r);
                            temp.4 = cmp::max(side.4, *r);
                        } else {
                            temp.3 = cmp::min(side.3, *r);
                            temp.4 = cmp::max(temp.4, *r);
                        }
                        new = Some(temp);
                    }
                }
            }

            for swap in to_swap.iter() {
                if let Some(swap) = swap {
                    sides.remove(&swap);
                }
            }

            if to_swap.len() > 0 {
                let new = new.unwrap();
                sides.insert(new);
            } else {
                sides.insert((is_col_move, *from_c, *c, *r, *r));
            }
        } else {
            let mut to_swap: Vec<Option<(bool, usize, usize, usize, usize)>> = vec![];
            let mut new: Option<(bool, usize, usize, usize, usize)> = None;
            let mut found = false; 
            
            for side in sides.iter() {
                let is_new_start = side.3.wrapping_sub(1) == *c;
                let is_new_end = side.4 + 1 == *c;
                if side.0 == is_col_move && side.1 == *from_r && side.2 == *r && (is_new_start || is_new_end) {
                    if !found {
                        found = true;
                        to_swap.push(Some(*side));
                        if is_new_start {
                            new = Some((side.0, side.1, side.2, *c, side.4));
                        } else {
                            new = Some((side.0, side.1, side.2, side.3, *c));
                        }
                    } else {
                        to_swap.push(Some(*side));
                        let mut temp = new.unwrap();
                        if is_new_start {
                            temp.3 = cmp::min(temp.3, *c);
                            temp.4 = cmp::max(side.4, *c);
                        } else {
                            temp.3 = cmp::min(side.3, *c);
                            temp.4 = cmp::max(temp.4, *c);
                        }
                        new = Some(temp);
                    }
                }
            }
            
            for swap in to_swap.iter() {
                if let Some(swap) = swap {
                    sides.remove(&swap);
                }
            }

            if to_swap.len() > 0 {
                let new = new.unwrap();
                sides.insert(new);
            } else {
                sides.insert((is_col_move, *from_r, *r, *c, *c));
            }
        }
    }
    sides.len()
}
