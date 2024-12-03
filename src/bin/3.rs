use std::{fs::File, io::Read, usize};

fn main() {
    let file_path = "./input/3_input.txt";
    let mut file = File::open(file_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    part_1(&content);
    part_2(&content);
}

fn part_1(content: &String) {
    let mut index = 0;
    let mut total = 0;
    while index < content.chars().count() {
        let end = (index + 3).min(content.len());
        let char = &content[index..end];
        if char == "mul" {
            let expression = &content[index..index + 12];
            total += try_calculate_expression(expression);
        }
        index += 1;
    }

    println!("{total}");
}

fn part_2(content: &String) {
    let mut index = 0;
    let mut total = 0;
    let mut do_muls = true;
    while index < content.chars().count() {
        let do_end = (index + 2).min(content.len());
        let char_at_ai = &content[index..do_end];
        if char_at_ai == "do" {
            let end = (index + 4).min(content.len());
            let instruction = &content[index..end];
            if instruction == "do()" {
                do_muls = true;
            }
            let end = (index + 7).min(content.len());
            let instruction = &content[index..end];
            if instruction == "don't()" {
                do_muls = false;
            }
        } else if do_muls {
            let end = (index + 3).min(content.len());
            let instruction = &content[index..end];
            if instruction == "mul" {
                let expression = &content[index..index + 12];
                total += try_calculate_expression(expression);
            }
        }
        index += 1;
    }

    println!("{total}");
}

fn try_calculate_expression(expression: &str) -> i32 {
    if expression.len() < 8 {
        return 0;
    }

    let expression = &expression[3..expression.len()];
    let str = String::from(expression);
    let mut i = 0;
    let mut start = usize::MAX;
    let mut end = 0;
    while i < str.chars().count() {
        let char = str.chars().nth(i).unwrap();

        if char != '(' && char != ')' {
            i += 1;
            continue;
        }

        if char == '(' && start == usize::MAX && end == 0 {
            start = i;
        } else if char == '(' && start == usize::MAX && end != 0 {
            return 0;
        } else if char == '(' && start != usize::MAX {
            return 0;
        }

        if char == ')' && start != usize::MAX && end == 0 {
            end = i;
            break;
        } else if char == ')' && start == usize::MAX && end != 0 {
            return 0;
        } else if char == ')' && end != 0 {
            return 0;
        }
        i += 1;
    }

    if start > end || end == 0 {
        return 0;
    }
    let expression = &expression[start..end + 1];

    if let Some((left, right)) = expression
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .map(|s| {
            let mut parts = s.split(',');
            (parts.next(), parts.next())
        })
    {
        if let (Some(left), Some(right)) = (left, right) {
            let left_num: i32 = left.parse().unwrap();
            let right_num: i32 = right.parse().unwrap();
            return left_num * right_num;
        }
    }
    0
}
