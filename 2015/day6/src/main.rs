use std::fs;

fn part1() {
    let mut total = 0;
    let mut grid = [[false; 1000]; 1000];
    let mut input = fs::read_to_string("input.txt").expect("Unable to read input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let lines = input.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        if instruction[1] == "off" {
            let start = instruction[2]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[4]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] = false;
                }
            }
        }
        else if instruction[1] == "on" {
            let start = instruction[2]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[4]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] = true;
                }
            }
        }
        else {
            let start = instruction[1]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[3]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] = !grid[i][j];
                }
            }
        }
    }

    for i in 0..1000 {
        for j in 0..1000{
            if grid[i][j] {
                total += 1;
            }
        }
    }

    println!("Part1 {}", total);

}

fn part2() {
    let mut grid = [[0 as u32; 1000]; 1000];
    let mut input = fs::read_to_string("input.txt").expect("Unable to read input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let lines = input.split("\n").collect::<Vec<&str>>();

    for line in lines {
        let instruction = line.split(" ").collect::<Vec<&str>>();
        if instruction[1] == "off" {
            let start = instruction[2]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[4]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] = grid[i][j].saturating_sub(1);
                }
            }
        }
        else if instruction[1] == "on" {
            let start = instruction[2]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[4]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] += 1;
                }
            }
        }
        else {
            let start = instruction[1]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            let end = instruction[3]
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();            
            for i in start[0]..=end[0] {
                for j in start[1]..=end[1] {
                    grid[i][j] += 2;
                }
            }
        }
    }

    let total = grid.iter().fold(0, |a, b| a + b.iter().sum::<u32>());

    println!("Part2 {}", total);

}
fn main() {
    part1();
    part2();
}
