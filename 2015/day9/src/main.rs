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
    let mut cities: HashSet<&str> = HashSet::new();
    let mut map: HashMap<(&str, &str), u64> = HashMap::new();
    let input = include_str!("../input.txt");

    for line in input.lines() {
        let parse = line.split(" ").collect::<Vec<&str>>();
        cities.insert(parse[0]);
        cities.insert(parse[2]);
        map.insert((parse[0], parse[2]), parse[4].parse::<u64>().unwrap());
        map.insert((parse[2], parse[0]), parse[4].parse::<u64>().unwrap());
    }

    let mut cities_list: Vec<&str> = Vec::new();
    for city in cities.iter() {
        cities_list.push(city);
    }
    let mut perm: Vec<Vec<&str>> = Vec::new();
    permutation(cities_list.len(), &mut cities_list, &mut perm);
    let mut ans = 99999;
    for p in perm {
        let mut total = 0;
        for pair in p.windows(2) {
            total += map.get(&(pair[0], pair[1])).unwrap();
        }
        ans = ans.min(total);
    }
    
    println!("Part1 {}", ans);
}

fn part2() {
    let mut cities: HashSet<&str> = HashSet::new();
    let mut map: HashMap<(&str, &str), u64> = HashMap::new();
    let input = include_str!("../input.txt");

    for line in input.lines() {
        let parse = line.split(" ").collect::<Vec<&str>>();
        cities.insert(parse[0]);
        cities.insert(parse[2]);
        map.insert((parse[0], parse[2]), parse[4].parse::<u64>().unwrap());
        map.insert((parse[2], parse[0]), parse[4].parse::<u64>().unwrap());
    }

    let mut cities_list: Vec<&str> = Vec::new();
    for city in cities.iter() {
        cities_list.push(city);
    }
    let mut perm: Vec<Vec<&str>> = Vec::new();
    permutation(cities_list.len(), &mut cities_list, &mut perm);
    let mut ans = 0;
    for p in perm {
        let mut total = 0;
        for pair in p.windows(2) {
            total += map.get(&(pair[0], pair[1])).unwrap();
        }
        ans = ans.max(total);
    }
    
    println!("Part2 {}", ans);
}
fn main() {
    part1();
    part2();
}
