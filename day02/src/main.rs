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
    score_map.insert(
        "A".to_string(),
        HashMap::new()
    );

    score_map["A"].insert(
        "X".to_string(),
        1
    );

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

fn main() {
    part1();
    part2();
}
