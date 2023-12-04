use std::collections::{HashMap, HashSet};

fn main() {
    let lines = include_str!("../input.txt")
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut total1 = 0;
    let mut records: Vec<(usize, usize)> = Vec::new();
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        let parse_str = line
            .split(": ")
            .into_iter()
            .map(|ele| ele.to_string())
            .collect::<Vec<String>>();

        let cards = parse_str[1]
            .split("| ")
            .into_iter()
            .map(|ele| ele.to_string())
            .collect::<Vec<String>>();

        let winning_cards: HashSet<u32> = HashSet::from_iter(
            cards[0]
                .split_whitespace()
                // .into_iter()
                .map(|ele| ele.parse::<u32>().unwrap())
                .into_iter(),
        );
        // .collect::<Vec<String>>();

        let user_cards: HashSet<u32> = HashSet::from_iter(
            cards[1]
                .split_whitespace()
                // .into_iter()
                .map(|ele| ele.parse::<u32>().unwrap())
                .into_iter(),
        );
        // .collect::<Vec<String>>();
        let p = user_cards
            .intersection(&winning_cards)
            .collect::<Vec<&u32>>()
            .len();
        if p > 0 {
            // println!("{}", p);
            total1 += 2u32.pow(p as u32 - 1);
        }

        let card_num = parse_str[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        records.push((card_num, p));
    }
    println!("{:?}", total1);

    let mut total2 = 0;
    let mut ans = [1usize; 200];

    for (card_num, count) in records.iter() {
        for _ in 0..ans[*card_num] {
            for i in card_num + 1..card_num + count + 1 {
                ans[i] += 1;
            }
        }
    }

    for (card_num, _) in records.iter() {
        total2 += ans[*card_num];
    }

    println!("{:?}", total2);
}
