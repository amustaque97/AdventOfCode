use std::collections::{HashSet, HashMap};

fn permutation<'a>(k: usize, a: &mut [&'a str], res: &mut Vec<Vec<&'a str>>) {
    if k == 1 {
        res.push(Vec::from(a));
    } else {
        permutation(k-1, a, res);

        for i in 0..k-1 {
            if k % 2 == 0 {
                a.swap(i, k-1);
            } else {
                a.swap(0, k-1);
            }
            permutation(k-1, a, res);
        }
    }
}

fn part1() {
    let mut persons: HashSet<&str> = HashSet::new();
    let mut map: HashMap<(&str, &str), i64> = HashMap::new();
    let input = include_str!("../input.txt");

    for line in input.lines() {
        let parse = line.strip_suffix(".").unwrap().split(" ").map(|x| x).collect::<Vec<&str>>();
        let n = parse.len();
        let mut happiness = parse[3].parse::<i64>().unwrap();
        if parse[2] == "lose" {
            happiness *= -1;
        }
        persons.insert(parse[0].clone());
        persons.insert(parse[n-1].clone());
        map.insert((parse[0].clone(), parse[n-1].clone()), happiness);
    }
    let mut persons_list:Vec<&str> = Vec::new();
    let mut perm: Vec<Vec<&str>> = Vec::new();
    for p in persons.iter() {
        persons_list.push(p);
    }
    permutation(persons.len(), &mut persons_list, &mut perm);
    
    let mut ans = 0;
    for p in perm {
        let mut total = 0;
        for pair in p.windows(2) {
            total += map.get(&(pair[0], pair[1])).unwrap();
            total += map.get(&(pair[1], pair[0])).unwrap();
        }
        total += map.get(&(p[0], p[p.len()-1])).unwrap();
        total += map.get(&(p[p.len()-1], p[0])).unwrap();
        ans = ans.max(total);
    }
    
    println!("Part1 {}", ans);

}

fn part2() {
    let mut persons: HashSet<&str> = HashSet::new();
    let mut map: HashMap<(&str, &str), i64> = HashMap::new();
    let input = include_str!("../input.txt");

    for line in input.lines() {
        let parse = line.strip_suffix(".").unwrap().split(" ").map(|x| x).collect::<Vec<&str>>();
        let n = parse.len();
        let mut happiness = parse[3].parse::<i64>().unwrap();
        if parse[2] == "lose" {
            happiness *= -1;
        }
        persons.insert(parse[0].clone());
        persons.insert(parse[n-1].clone());
        map.insert((parse[0].clone(), "you"), 0);
        map.insert(("you", parse[n-1].clone()), 0);
        map.insert((parse[0].clone(), parse[n-1].clone()), happiness);
    }
    let mut persons_list:Vec<&str> = Vec::new();
    let mut perm: Vec<Vec<&str>> = Vec::new();
    for p in persons.iter() {
        persons_list.push(p);
    }
    persons_list.push("you");
    permutation(persons.len(), &mut persons_list, &mut perm);
    
    let mut ans = 0;
    for p in perm {
        let mut total = 0;
        for pair in p.windows(2) {
            total += map.get(&(pair[0], pair[1])).unwrap();
            total += map.get(&(pair[1], pair[0])).unwrap();
        }
        total += map.get(&(p[0], p[p.len()-1])).unwrap();
        total += map.get(&(p[p.len()-1], p[0])).unwrap();
        ans = ans.max(total);
    }
    
    println!("Part2 {}", ans);

}
fn main() {
    part1();
    part2();
}
