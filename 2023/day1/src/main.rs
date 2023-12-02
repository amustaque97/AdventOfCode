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

fn part2() {
    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];
    let file = include_str!("../input.txt")
        .split("\n")
        .collect::<Vec<&str>>();
    let mut total = 0;

    for line in &file {
        if line.is_empty() {
            continue;
        }
        let mut first = None;
        'out: for i in 0..line.len() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if line[i..i + num.len()] == **num {
                    first = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let Some(first) = first else {
                panic!("invalid input");
            };

        let mut last = None;
        'out: for i in (0..line.len()).rev() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if line[i..i + num.len()] == **num {
                    last = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let Some(last) = last else {
                panic!("invalid input");
            };

        total += 10 * first as i32 + last as i32;
    }

    println!("{}", total);
}

fn main() {
    part1();
    part2();
}
