use std::fs;

fn convert_to_prio(byte: u8) -> i32 {
    if byte > 96 {
        return byte as i32 - 96;
    }

    byte as i32 - 38
}

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut total = 0;

    for line in lines {
        let bytes = line.as_bytes();
        let mut found_bytes: Vec<u8> = Vec::new();
        for i_byte in &bytes[..bytes.len()/2] {
            if found_bytes.iter().any(|v| *v == *i_byte) {
                continue;
            }
            found_bytes.push(*i_byte);

            for j_byte in &bytes[bytes.len()/2..] {
                if j_byte != i_byte {
                    continue;
                }

                let i_prio = convert_to_prio(*i_byte);
                total += i_prio;
                break;
            }
        }
    }

    println!("Part1: {}", total);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");

    let lines = content.lines();

    let mut total: i32 = 0;

    let mut group: Vec<u8> = Vec::new();

    let mut i = 0;
    for line in lines {
        let bytes = line.as_bytes();
        if i % 3 == 0 {
            group.clear();
            group.extend(bytes);
            i += 1;
            continue;
        }

        let mut new_group: Vec<u8> = Vec::new();
        for byte in bytes {
            if group.iter().any(|v| *v == *byte) {
                new_group.push(*byte);
            }
        }

        group = new_group;

        if i % 3 == 2 {
            total += convert_to_prio(group[0]);
        }

        i += 1;
    }

    println!("Part2: {}", total);
}

fn main() {
    part1();
    part2();
}
