fn part1() {
    let file = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();
    let mut ans: Vec<u32> = Vec::new();

    for word in file {
        if word.is_empty() {
            continue;
        }
        let chrs: Vec<char> = word.chars().into_iter().collect();
        let mut left = 0;
        let mut right = chrs.len() - 1;

        while left <= right {
            if chrs[left].is_digit(10) && chrs[right].is_digit(10) {
                let num: u32 = format!("{}{}", chrs[left], chrs[right]).parse().unwrap();
                // println!("Found a number in word {} => {}", word, num);
                ans.push(num);
                break;
            } else if !chrs[left].is_digit(10) {
                left += 1;
            } else if !chrs[right].is_digit(10) {
                right -= 1;
            } else {
                left += 1;
                right -= 1;
            }
        }
    }

    // println!("{:?}", ans);

    println!("{}", ans.iter().sum::<u32>());
}

fn main() {
    part1();
}
