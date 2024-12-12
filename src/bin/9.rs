use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let file_path = "./input/9_input.txt";
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let mut nums = vec![];
    for ch in content.chars() {
        if let Some(num) = ch.to_digit(10) {
            nums.push(num);
        }
    }

    let mut counter = 0;
    let mut id_counter = 0;
    let mut blocks: Vec<(u32, Option<u32>)> = vec![];
    for num in nums {
        if counter % 2 == 0 {
            blocks.push((num, Some(id_counter)));
            id_counter += 1;
        } else {
            blocks.push((num, None));
        }
        counter += 1;
    }

    part_1(blocks.clone());
    part_2(blocks.clone());
}

fn part_1(mut blocks: Vec<(u32, Option<u32>)>) {
    let mut right = blocks.len() - 1;
    let mut compacted_blocks: Vec<u32> = vec![];
    for i in 0..blocks.len() {
        let (len, id) = blocks[i];
        if let Some(id) = id {
            for _ in 0..len {
                compacted_blocks.push(id);
            }
        } else {
            if right <= i {
                break;
            }
            for _ in 0..len {
                if blocks[right].0 > 0 {
                    compacted_blocks.push(blocks[right].1.unwrap());
                    blocks[right].0 -= 1;
                } else {
                    right -= 2;
                    if right <= i {
                        break;
                    }
                    compacted_blocks.push(blocks[right].1.unwrap());
                    blocks[right].0 -= 1;
                }
            }
        }
    }

    let mut counter: u64 = 0;
    let mut checksum: u64 = 0;
    for n in compacted_blocks {
        checksum += counter * n as u64;
        counter += 1;
    }

    println!("{}", checksum);
}

fn part_2(mut blocks: Vec<(u32, Option<u32>)>) {
    for i in (0..blocks.len()).rev() {
        let (len, id) = blocks[i];
        if id == None {
            continue;
        }

        for j in 0..=i {
            if blocks[j].0 >= len && blocks[j].1 == None {
                let temp = blocks[j];
                blocks[j] = blocks[i];
                blocks[i] = (len, None);
                let remaining = temp.0 - blocks[j].0;
                if remaining > 0 {
                    blocks.insert(j + 1, (remaining, None));
                }
                break;
            }
        }
    }

    let mut compacted_blocks: Vec<Option<u32>> = vec![];
    for (len, id) in blocks {
        for _ in 0..len {
            compacted_blocks.push(id);
        }
    }

    let mut counter: u64 = 0;
    let mut checksum: u64 = 0;
    for id in compacted_blocks {
        if let Some(id) = id {
            checksum += counter * id as u64;
        }
        counter += 1;
    }

    println!("{}", checksum);
}
