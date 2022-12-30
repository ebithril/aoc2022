use std::fs;
use std::collections::VecDeque;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();
    let mut piles: [VecDeque<char>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for line in lines {
        let mut should_move = true;
        for i in line.chars().enumerate().filter(|(_, c)| *c == '[').map(|(i, _)| i) {
            let pile = i / 4;
            let container = line.chars().nth(i + 1).unwrap();

            piles[pile].push_front(container);
            should_move = false;
        }

        if !should_move {
            continue;
        }

        if !line.starts_with("move") {
            continue;
        }


        let whitespace_indicies: Vec<usize> = line.chars().enumerate().filter(|(_, c)| *c == ' ').map(|(i, _)| i).collect();
        let number_to_move: usize = (&line[whitespace_indicies[0]+1..whitespace_indicies[1]]).parse().unwrap();
        let from_pile: usize = (&line[whitespace_indicies[2]+1..whitespace_indicies[3]]).parse().unwrap();
        let to_pile: usize = (&line[whitespace_indicies[4]+1..]).parse().unwrap();

        for _ in 0..number_to_move {
            let c = piles[from_pile-1].pop_back().unwrap();
            piles[to_pile-1].push_back(c);
        }
    }

    let mut result = String::from("");

    for pile in piles {
        result.push(*pile.back().unwrap());
    }

    println!("Part1: {}", result);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();
    let mut piles: [VecDeque<char>; 9] = [
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for line in lines {
        let mut should_move = true;
        for i in line.chars().enumerate().filter(|(_, c)| *c == '[').map(|(i, _)| i) {
            let pile = i / 4;
            let container = line.chars().nth(i + 1).unwrap();

            piles[pile].push_front(container);
            should_move = false;
        }

        if !should_move {
            continue;
        }

        if !line.starts_with("move") {
            continue;
        }


        let whitespace_indicies: Vec<usize> = line.chars().enumerate().filter(|(_, c)| *c == ' ').map(|(i, _)| i).collect();
        let number_to_move: usize = (&line[whitespace_indicies[0]+1..whitespace_indicies[1]]).parse().unwrap();
        let from_pile: usize = (&line[whitespace_indicies[2]+1..whitespace_indicies[3]]).parse().unwrap();
        let to_pile: usize = (&line[whitespace_indicies[4]+1..]).parse().unwrap();

        let mut to_move: Vec<char> = Vec::new();

        for _ in 0..number_to_move {
            let c = piles[from_pile-1].pop_back().unwrap();
            to_move.push(c);
        }

        for c in to_move.iter().rev() {
            piles[to_pile-1].push_back(*c);
        }
    }

    let mut result = String::from("");

    for pile in piles {
        result.push(*pile.back().unwrap());
    }

    println!("Part2: {}", result);
}

fn main() {
    part1();
    part2();
}
