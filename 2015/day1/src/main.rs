use std::fs;

fn part1() {
    let mut res = 0;
    let mut input = fs::read_to_string("input1.txt").expect("Unable to open input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let brackets = input.chars();

    for ch in brackets {
        if ch == '(' {
            res += 1;
            } else {
            res -= 1;
            }

    }
    
    println!("Part1 {}", res);
}

fn part2() {
    let mut res = 0;
    let mut input = fs::read_to_string("input1.txt").expect("Unable to open input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let brackets = input.chars();

    let mut position = 1;

    for ch in brackets {
        if ch == '(' {
            res += 1;
            } else {
            res -= 1;
            }
        if res == -1 {
            break;
        } else {
            position += 1;
        }
    }

    println!("Part2 {}", position);
}

fn main() {
    part1();
    part2();

}
