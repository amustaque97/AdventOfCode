use std::str;

fn part1() {
    let mut total = 0;
    let mut total_memory = 0;
    let input = include_str!("../input.txt");

    for line in input.lines() {
        total += line.len();
        let mut index = 1;
        let char = line.chars().collect::<Vec<char>>();
        while index < char.len()-1 {
            total_memory +=  1;
            if char[index] == '\\' {
                if char[index+1] == 'x' {
                    index+=4;
                } else {
                    index+=2;
                }
            } else {
                index += 1;
            }
        }

        println!("Part1 {}", total - total_memory);

    }
}
fn main() {
    part1();
}
