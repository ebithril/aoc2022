use std::fs;
use std::collections::VecDeque;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let mut sequence: VecDeque<char> = VecDeque::new();

    for (i, c) in content.chars().enumerate() {
        for (j, oc) in sequence.iter().enumerate() {
            if *oc == c {
                for _ in 0..j+1 {
                    sequence.pop_front();
                }
                break;
            }
        }

        sequence.push_back(c);

        if sequence.len() == 4 {
            println!("Part1: {}", i + 1);
            return;
        }
    }
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
}

fn main() {
    part1();
    part2();
}
