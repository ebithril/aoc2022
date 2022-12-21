use std::fs;
use std::collections::HashMap;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut total_score:i32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }

        let mut round_score = 1;
        if line.contains("Y") {
            round_score = 2;
        }
        else if line.contains("Z") {
            round_score = 3;
        }

        if line.contains("A") {
            if round_score == 1 {
                round_score += 3;
            }
            else if round_score == 2 {
                round_score += 6;
            }
        }
        else if line.contains("B") {
            if round_score == 2 {
                round_score += 3;
            }
            else if round_score == 3 {
                round_score += 6;
            }
        }
        else if line.contains("C") {
            if round_score == 3 {
                round_score += 3;
            }
            else if round_score == 1 {
                round_score += 6;
            }
        }

        total_score += round_score;
    }

    println!("{}", total_score);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();
    let mut score_map:HashMap<String, HashMap<String, i32>> = HashMap::new();

    // A, rock 1
    // B, paper 2
    // C, scissors 3

    score_map.insert(
        "A".to_string(),
        HashMap::from([
            ("X".to_string(), 3),
            ("Y".to_string(), 4),
            ("Z".to_string(), 8),
        ])
    );

    score_map.insert(
        "B".to_string(),
        HashMap::from([
            ("X".to_string(), 1),
            ("Y".to_string(), 5),
            ("Z".to_string(), 9),
        ])
    );

    score_map.insert(
        "C".to_string(),
        HashMap::from([
            ("X".to_string(), 2),
            ("Y".to_string(), 6),
            ("Z".to_string(), 7),
        ])
    );


    let mut total_score:i32 = 0;
    for line in lines {
        if line == "" {
            continue;
        }

        let instructions = line.split_whitespace().collect::<Vec<&str>>();

        total_score += score_map[instructions[0]][instructions[1]];
    }

    println!("{}", total_score);
}

fn main() {
    part1();
    part2();
}
