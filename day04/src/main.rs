use std::fs;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut total_pairs: i32 = 0;
    for line in lines {
        let sections: Vec<&str> = line.split(",").collect();
        let first_section: Vec<&str> = sections[0].split("-").collect();
        let first_min: i32 = first_section[0].parse().unwrap();
        let first_max: i32 = first_section[1].parse().unwrap();

        let second_section: Vec<&str> = sections[1].split("-").collect();
        let second_min: i32 = second_section[0].parse().unwrap();
        let second_max: i32 = second_section[1].parse().unwrap();

        if first_min <= second_min && first_max >= second_max{
            total_pairs += 1;
        }
        else if second_min <= first_min && second_max >= first_max {
            total_pairs += 1;
        }
    }

    println!("Part1: {}", total_pairs)
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut total_pairs: i32 = 0;
    for line in lines {
        let sections: Vec<&str> = line.split(",").collect();
        let first_section: Vec<&str> = sections[0].split("-").collect();
        let first_min: i32 = first_section[0].parse().unwrap();
        let first_max: i32 = first_section[1].parse().unwrap();

        let second_section: Vec<&str> = sections[1].split("-").collect();
        let second_min: i32 = second_section[0].parse().unwrap();
        let second_max: i32 = second_section[1].parse().unwrap();

        if first_min > second_max {
            continue;
        }

        if second_min > first_max {
            continue;
        }

        total_pairs += 1;
    }

    println!("Part2: {}", total_pairs)
}

fn main() {
    part1();
    part2();
}
