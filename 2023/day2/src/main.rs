#[derive(Debug)]
struct Game {
    blue: u32,
    red: u32,
    green: u32,
}

fn part1() {
    let lines = include_str!("../input.txt").split("\n").collect::<Vec<_>>();

    let mut game_list: Vec<&str> = Vec::new();

    'out: for line in &lines {
        if line.is_empty() {
            continue;
        }
        let parse_str = line.split(": ").collect::<Vec<_>>();
        let cubes_iter = parse_str[1].split("; ").collect::<Vec<_>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cubes in cubes_iter {
            let c = cubes.split(", ").collect::<Vec<_>>();
            for ball in c {
                if ball.contains("red") {
                    let t = ball.strip_suffix(" red").unwrap().parse::<u32>().unwrap();
                    if t > 12 {
                        continue 'out;
                    }
                    red += t;
                } else if ball.contains("blue") {
                    let t = ball.strip_suffix(" blue").unwrap().parse::<u32>().unwrap();
                    if t > 14 {
                        continue 'out;
                    }
                    blue += t;
                } else if ball.contains("green") {
                    let t = ball.strip_suffix(" green").unwrap().parse::<u32>().unwrap();
                    if t > 13 {
                        continue 'out;
                    }
                    green += t;
                }
            }
        }
        // let game = Game { red, green, blue };
        // println!("{} => {:?}", parse_str[0], game);

        game_list.push(parse_str[0]);
    }

    let ans = game_list
        .iter()
        .map(|&game| game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap())
        .sum::<u32>();

    println!("{:?}", ans);
}

fn part2() {
    let lines = include_str!("../input.txt").split("\n").collect::<Vec<_>>();

    let mut total = 0;

    for line in &lines {
        if line.is_empty() {
            continue;
        }
        let parse_str = line.split(": ").collect::<Vec<_>>();
        let cubes_iter = parse_str[1].split("; ").collect::<Vec<_>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for cubes in cubes_iter {
            let c = cubes.split(", ").collect::<Vec<_>>();
            for ball in c {
                if ball.contains("red") {
                    let t = ball.strip_suffix(" red").unwrap().parse::<u32>().unwrap();
                    red = red.max(t);
                } else if ball.contains("blue") {
                    let t = ball.strip_suffix(" blue").unwrap().parse::<u32>().unwrap();
                    blue = blue.max(t);
                } else if ball.contains("green") {
                    let t = ball.strip_suffix(" green").unwrap().parse::<u32>().unwrap();
                    green = green.max(t);
                }
            }
        }
        let game = Game { red, green, blue };
        // println!("{} => {:?}", parse_str[0], game);
        total += game.red * game.blue * game.green;
    }

    println!("{:?}", total);
}

fn main() {
    part1();
    part2();
}
