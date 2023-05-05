#[allow(dead_code, unused_assignments)]
#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest_seconds: u32,
}

impl Reindeer {
    pub fn calculate(&self, n: u32) -> u32 {
        let count = n / (self.rest_seconds + self.duration);
        let remainder = n % (self.rest_seconds + self.duration);

        count * self.speed * self.duration + self.speed * remainder.min(self.duration)
    }
}
pub fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect()
}

fn part1() {
    let lines: Vec<&str> = get_input();

    for line in lines {
        let input: Vec<&str> = line.split(" ").collect();
        let x: Reindeer = Reindeer {
            name: input[0].to_string(),
            speed: input[3].parse::<u32>().unwrap(),
            duration: input[6].parse::<u32>().unwrap(),
            rest_seconds: input[13].parse::<u32>().unwrap(),
        };
        println!("{:?}", x.calculate(2503))
    }
}

fn part2() {
    let lines: Vec<&str> = get_input();
    let mut players: Vec<Reindeer> = Vec::new();
    for line in &lines {
        let input: Vec<&str> = line.split(" ").collect();
        let x: Reindeer = Reindeer {
            name: input[0].to_string(),
            speed: input[3].parse::<u32>().unwrap(),
            duration: input[6].parse::<u32>().unwrap(),
            rest_seconds: input[13].parse::<u32>().unwrap(),
        };
        players.push(x);
    }

    let mut far: Vec<u32> = vec![0; players.len()];

    for sec in 1..=1000 {
        let mut furthest = 0;
        let mut max_dist = 0;
        for (idx, d) in players.iter().enumerate() {
            let dist = d.calculate(sec);
            if dist > max_dist {
                max_dist = dist;
                furthest = idx;
            }
        }
        far[furthest] += 1;
    }

    println!("{}", far.iter().max().unwrap());
}

fn main() {
    part1();
    part2();
}
