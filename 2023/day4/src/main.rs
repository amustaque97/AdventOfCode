use std::collections::HashSet;

fn main() {
    let lines = include_str!("../input.txt")
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let mut total1 = 0;
    for line in lines {
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
        let mut p = user_cards
            .intersection(&winning_cards)
            .collect::<Vec<&u32>>()
            .len();
        if p > 0 {
            p -= 1;
            // println!("{}", p);
            total1 += 2u32.pow(p.try_into().unwrap());
        }

    }
    println!("{:?}", total1);
}
