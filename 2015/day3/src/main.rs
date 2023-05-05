use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

struct Point {
    x: i32,
    y: i32
}

fn part1() {
    let mut p = Point{
        x: 0,
        y: 0,
    };
    let mut res: Vec<String> = Vec::new();
    let mut input = fs::read_to_string("input.txt").expect("Unable to read input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let dir = input.chars();

    // println!("{:?}", dir);

    // parse directions
    for d in dir {
        if d == '^' {
            p.y += 1;
        } else if d == 'v' {
            p.y -= 1;
        } else if d == '>' {
            p.x += 1;
        } else if d == '<' {
            p.x -= 1;
        }
        
        let ele = format!("{}:{}", p.x, p.y);

        if res.contains(&ele.to_string()) {
            continue;
        } else {
            res.push(ele.to_string());
        }
        
    }

    let mut houses = res.len();
    if p.x != 0 || p.y != 0 {
        houses += 1;
    }

    println!("Part1 {}", houses);
}

fn part2() {
    let mut s = Point{
        x: 0,
        y: 0,
    };
    let mut r = Point{
        x: 0,
        y: 0,
    };
    let mut map: HashMap<String, u32> = HashMap::new();
    let mut input = fs::read_to_string("input.txt").expect("Unable to read input file");
    input = input.strip_suffix("\n").unwrap().to_string();
    let dir = input.chars();

    // println!("{:?}", dir);

    // parse directions
    let mut toggle = false;
    for d in dir {
        if toggle {
            if d == '^' {
                s.y += 1;
            } else if d == 'v' {
                s.y -= 1;
            } else if d == '>' {
                s.x += 1;
            } else if d == '<' {
                s.x -= 1;
            }
            
            let ele = format!("{}:{}", s.x, s.y);
            map.entry(ele).and_modify(|x| *x += 1).or_insert(1);

        } else {
            if d == '^' {
                r.y += 1;
            } else if d == 'v' {
                r.y -= 1;
            } else if d == '>' {
                r.x += 1;
            } else if d == '<' {
                r.x -= 1;
            }
            
            let ele = format!("{}:{}", r.x, r.y);
            map.entry(ele).and_modify(|x| *x += 1).or_insert(1);

        }
        toggle = !toggle;
        
    }

    let houses = map.len();
    /*if s.x != 0 || s.y != 0 || r.x != 0 || r.y != 0  {
        houses += 1; // add (0,0) house - initial location
    }*/

    println!("Part2 {}", houses);
}


fn main() {
    part1();
    part2();
}
