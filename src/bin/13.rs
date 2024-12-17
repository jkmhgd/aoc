use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn main() {
    let path = "./input/13_input.txt";
    let input_path = Path::new(path);

    let file = File::open(&input_path).unwrap();
    let reader = BufReader::new(file);

    let mut m_part_1: Vec<ClawMachine> = Vec::new();
    let mut m_part_2: Vec<ClawMachine> = Vec::new();

    let mut current_button_a: Option<Button> = None;
    let mut current_button_b: Option<Button> = None;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.starts_with("Button A:") {
            let (x, y) = parse_button_coordinates(&line);
            current_button_a = Some(Button { x, y });
        } else if line.starts_with("Button B:") {
            let (x, y) = parse_button_coordinates(&line);
            current_button_b = Some(Button { x, y });
        } else if line.starts_with("Prize:") {
            let (x, y) = parse_prize_coordinates(&line);

            if let (Some(button_a), Some(button_b)) = (&current_button_a, &current_button_b) {
                m_part_1.push(ClawMachine {
                    button_a: button_a.clone(),
                    button_b: button_b.clone(),
                    prize: Prize { x, y },
                });

                m_part_2.push(ClawMachine {
                    button_a: button_a.clone(),
                    button_b: button_b.clone(),
                    prize: Prize {
                        x: x + 10000000000000,
                        y: y + 10000000000000,
                    },
                });

                current_button_a = None;
                current_button_b = None;
            }
        }
    }

    let mut total_tokens = 0;
    for machine in m_part_1.iter() {
        total_tokens += calc_min_tokens(machine);
    }
    println!("Part 1: {}", total_tokens);

    let mut total_tokens = 0;
    for machine in m_part_2.iter() {
        if let Some(min_tokens) = calc_min_tokens_part_2(machine) {
            total_tokens += min_tokens;
        }
    }
    println!("Part 2: {}", total_tokens);
}

fn parse_button_coordinates(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split([':', ',', '+']).collect();
    let x = parts[2].trim().parse::<i64>().unwrap();
    let y = parts[4].trim().parse::<i64>().unwrap();
    (x, y)
}

fn parse_prize_coordinates(line: &str) -> (i64, i64) {
    let parts: Vec<&str> = line.split(['=', ',']).collect();
    let x = parts[1].trim().parse::<i64>().unwrap();
    let y = parts[3].trim().parse::<i64>().unwrap();
    (x, y)
}

fn calc_min_tokens(machine: &ClawMachine) -> i64 {
    let mut solutions: Vec<i64> = vec![];
    for a in 0..=100 {
        for b in 0..=100 {
            let (a_x, a_y) = (a * machine.button_a.x, a * machine.button_a.y);
            let (b_x, b_y) = (b * machine.button_b.x, b * machine.button_b.y);
            if machine.prize.x == a_x + b_x && machine.prize.y == a_y + b_y {
                let tokens = (a * 3) + (b * 1);
                solutions.push(tokens);
            }
        }
    }

    if solutions.len() > 0 {
        return *solutions.iter().min().unwrap();
    }
    0
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

// Check if two pairs (x1,y1) and (x2,y2) are collinear and if so, return the factor c such that (x2,y2)=c*(x1,y1)
fn collinearity_factor(x1: i64, y1: i64, x2: i64, y2: i64) -> Option<f64> {
    if x1 == 0 && y1 == 0 {
        return None;
    }
    if x1 == 0 {
        if x2 == 0 {
            return Some(y2 as f64 / y1 as f64);
        } else {
            return None;
        }
    }
    if x1 * y2 == y1 * x2 {
        Some(x2 as f64 / x1 as f64)
    } else {
        None
    }
}

// Got some LLM help with this function.
fn calc_min_tokens_part_2(machine: &ClawMachine) -> Option<i64> {
    let (xa, ya) = (machine.button_a.x, machine.button_a.y);
    let (xb, yb) = (machine.button_b.x, machine.button_b.y);
    let (xt, yt) = (machine.prize.x, machine.prize.y);

    let delta = xa * yb - ya * xb;

    if delta != 0 {
        let a_num = xt * yb - yt * xb;
        let b_num = -xt * ya + yt * xa;

        if a_num % delta != 0 || b_num % delta != 0 {
            return None;
        }

        let a = a_num / delta;
        let b = b_num / delta;

        if a >= 0 && b >= 0 {
            return Some(3 * a + b);
        } else {
            return None;
        }
    } else {
        if (xa, ya) == (0,0) {
            if (xt, yt) == (0,0) {
                return Some(0);
            } else {
                return None;
            }
        }

        if let Some(_) = collinearity_factor(xa, ya, xb, yb) {
            if let Some(_k) = collinearity_factor(xa, ya, xt, yt) {
                if xa == 0 && ya == 0 {
                    return if xt == 0 && yt == 0 { Some(0) } else { None };
                }

                if xa != 0 {
                    if xb * ya != yb * xa {
                        return None;
                    }
                    if xt * ya != yt * xa {
                        return None;
                    }

                    if xt % xa != 0 || yt % ya != 0 {
                        return None;
                    }
                    let k_int = xt / xa;

                    if k_int < 0 {
                        return None;
                    }

                    let g2 = gcd(xb, xa);
                    let m_num = xb / g2;
                    let m_den = xa / g2;

                    if m_num == 0 {
                        return if k_int >= 0 {
                            Some(3 * k_int)
                        } else {
                            None
                        };
                    }

                    let upper_t = if k_int < 0 {
                        return None;
                    } else {
                        (k_int as f64 / m_num as f64).floor() as i64
                    };

                    if upper_t < 0 {
                        return None;
                    }

                    let factor = 3 * m_num - m_den;
                    let choose_t = if factor > 0 {
                        upper_t
                    } else if factor < 0 {
                        0
                    } else {
                        0
                    };

                    let b = m_den * choose_t;
                    let a = k_int - (m_num * b) / m_den;

                    if a < 0 || b < 0 {
                        return None;
                    }

                    return Some(3 * a + b);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}
