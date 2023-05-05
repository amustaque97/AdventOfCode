use std::fs;

fn part1() {

    let mut total = 0;

    let input = fs::read_to_string("input1.txt").expect("Unable to read input file");
    let lines = input.split("\n").collect::<Vec<&str>>();

    for line in lines {
        if line == "" {
            break;
        }
        let i = line.split("x").collect::<Vec<&str>>();
        let length = i[0].parse::<i32>().unwrap();
        let width = i[1].parse::<i32>().unwrap();
        let height = i[2].parse::<i32>().unwrap();

        let area = vec![length*width, width*height, height*length];
        let min_value = area.iter().min().unwrap();

        for val in &area {
            total += 2*val;
        }

        total += min_value;
    }

    println!("Part1 {}", total);

}

fn part2() {
    let mut total = 0;

    let input = fs::read_to_string("input1.txt").expect("Unable to read input file");
    let lines = input.split("\n").collect::<Vec<&str>>();

    for line in lines {
        if line == "" {
            break;
        }
        let i = line.split("x").collect::<Vec<&str>>();
        let mut sides = vec![ i[0].parse::<i32>().unwrap(), 
                        i[1].parse::<i32>().unwrap(),
                        i[2].parse::<i32>().unwrap()
                        ];
        sides.sort();

        total += 2*sides[0] + 2*sides[1];
        total += sides[0] * sides[1] * sides[2];

    }

    println!("Part2 {}", total);

}

fn main() {
    part1();
    part2();
}
