use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file_path = "./input/2_input.txt";
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut safe_reports = 0;
    for report in reader.lines() {
        let mut levels: Vec<u32> = report
            .unwrap()
            .split(' ')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();

        let (bad_level, is_safe) = is_safe_report(&levels);
        if is_safe {
            safe_reports += 1;
        } else {
            let val = levels.remove(bad_level);
            let (_, is_safe) = is_safe_report(&levels);
            if is_safe {
                safe_reports += 1;
                continue;
            }
            levels.insert(bad_level, val);

            let level_to_remove = if bad_level > 1 { bad_level - 1 } else { 0 };
            let val = levels.remove(level_to_remove);
            let (_, is_safe) = is_safe_report(&levels);
            if is_safe {
                safe_reports += 1;
                continue;
            }
            levels.insert(level_to_remove, val);

            let level_to_remove = if bad_level < levels.len() {
                bad_level + 1
            } else {
                levels.len()
            };
            let _ = levels.remove(level_to_remove);
            let (_, is_safe) = is_safe_report(&levels);
            if is_safe {
                safe_reports += 1;
                continue;
            }
        }
    }

    println!("{safe_reports}");
}

fn is_safe_report(levels: &Vec<u32>) -> (usize, bool) {
    let mut prev_level = levels[0];
    let is_increasing = levels[0] < levels[1];

    for (i, level) in levels.iter().skip(1).enumerate() {
        if is_bad_level(*level, prev_level, is_increasing) {
            return (i, false);
        } else {
            prev_level = *level;
        }
    }

    (0, true)
}

fn is_bad_level(level: u32, prev_level: u32, is_increasing: bool) -> bool {
    let increased = level > prev_level;
    let diff = level.abs_diff(prev_level);
    (increased && !is_increasing) || (!increased && is_increasing) || (diff < 1 || diff > 3)
}
