#![allow(dead_code, unused)]

#[derive(Debug)]
struct Ingredient {
    c: i64,
    d: i64,
    f: i64,
    t: i64,
    cal: i64,
}

impl Ingredient {
    pub fn calc(&self, n: i64) -> (i64, i64, i64, i64) {
        (self.c * n, self.d * n, self.f * n, self.t * n)
    }
}

fn part1() {
    let lines: Vec<&str> = include_str!("../input.txt")
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect();

    let mut ingredients: Vec<Ingredient> = Vec::new();

    for line in lines {
        let input: Vec<&str> = line.split(", ").collect();
        let c: Vec<&str> = input[0].split(": ").collect();
        let cap_str: Vec<&str> = c[1].split(" ").collect(); // .collect().get(1).parse::<i64>();
        let cap = cap_str[1].parse::<i64>().unwrap();
        // println!("{:?}", input[1].split(" ").collect::<Vec<&str>>());
        let i = Ingredient {
            c: cap,
            d: input[1].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap(),
            f: input[2].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap(),
            t: input[3].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap(),
            cal: input[4].split(" ").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap(),
        };

        ingredients.push(i);
    }

    // println!("{:?}", ingredients);

    let mut ans = 0;
    let mut part2 = 0;

    for i in 1..=100 {
        for j in 1..=(100 - i) as i64 {
            for k in 1..=(100 - i - j) as i64 {
                let l = 100 - (i + j + k) as i64;
                if l < 0 {
                    continue;
                }
                let x = ingredients.get(0).unwrap();
                let y = ingredients.get(1).unwrap();
                let z = ingredients.get(2).unwrap();
                let a = ingredients.get(3).unwrap();

                let mut c = i * x.c + j * y.c + k * z.c + l * a.c;
                if c < 0 {
                    c = 0;
                }

                let mut d = i * x.d + j * y.d + k * z.d + l * a.d;
                if d < 0 {
                    d = 0;
                }

                let mut f = i * x.f + j * y.f + k * z.f + l * a.f;
                if f < 0 {
                    f = 0;
                }

                let mut t = i * x.t + j * y.t + k * z.t + l * a.t;
                if t < 0 {
                    t = 0;
                }

                let calc = c * d * f * t;
                if ans < calc {
                    ans = calc;
                }

                let cal = i * x.cal + j * y.cal + k * z.cal + l * a.cal;
                if cal == 500 {
                    part2 = part2.max(calc);
                }

            }
        }
    }

    println!("{ans}");
    println!("{part2}");
}

fn main() {
    part1();
}
