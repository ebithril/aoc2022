use std::fs;

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut current:i32 = 0;
    let mut max:i32 = 0;

    for line in lines {
        if line == "" {
            if current > max {
                max = current;
            }

            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap();
    }

    println!("{}", max);
}

fn part2() {
    let content = fs::read_to_string("input.txt").expect("Expected to read file");
    let lines = content.lines();

    let mut current:i32 = 0;
    let mut maxes:[i32; 3] = [0; 3];
    let mut min_index = 0;

    for line in lines {
        if line == "" {
            if current > maxes[min_index] {
                maxes[min_index] = current;

                let mut new_min_index = 0;
                for i in 1..3 {
                    if maxes[i] < maxes[new_min_index] {
                        new_min_index = i;
                    }
                }

                min_index = new_min_index;
            }

            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap();
    }

    println!("{}", maxes[0] + maxes[1] + maxes[2]);
}

fn main() {
    part1();
    part2();
}
