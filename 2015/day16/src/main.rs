fn part1() {
    let lines: Vec<&str> = include_str!("../input.txt")
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect();

    let items = vec![
        r"children: 3",
        r"cats: 7",
        r"samoyeds: 2",
        r"pomeranians: 3",
        r"akitas: 0",
        r"vizslas: 0",
        r"goldfish: 5",
        r"trees: 3",
        r"cars: 2",
        r"perfumes: 1",
    ];

    let mut ans = 0;

    'outer: for (idx, line) in lines.iter().enumerate() {
        for itm in &items {
            let thing: &str = itm.split(": ").collect::<Vec<&str>>()[0];
            if line.contains(thing) && !line.contains(itm) {
                continue 'outer;
            }
        }
        // println!("{itm} => {line}");
        println!("{line}");
        ans = idx;
    }

    println!("{ans}");
}

fn main() {
    part1();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            "Sue 96: cars: 2, trees: 9, samoyeds: 4".contains(r"cars: 2"),
            true
        )
    }
    #[test]
    fn it_does_not_works() {
        assert_eq!(
            "Sue 96: cars: 5, trees: 9, samoyeds: 4".contains(r"cars: 2"),
            false
        )
    }
}
