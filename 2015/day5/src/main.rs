use std::fs;


fn part1() {
    let mut total = 0;
    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let words = input.split("\n").collect::<Vec<&str>>();
    
    let vowels = "aeiou";
    for word in words {
        if word == "" {
            continue;
        }
        let mut vowels_count = 0;
        let mut found_double_ch = false;
        let mut found_restricted_word = false;
        for ch in word.chars() {
            // first condition
            if vowels.contains(ch) {
                vowels_count += 1;
            }
            // second condition
            let double_chars = format!("{}{}", ch, ch);
            if word.contains(&double_chars) && found_double_ch == false {
                found_double_ch = true;
            }
        }
        // third condition
        for not_contain in vec!["ab", "cd", "pq", "xy"] {
            if word.contains(&not_contain) {
                found_restricted_word = true;
                break;
            }
        }

        if vowels_count >= 3 && found_double_ch == true && found_restricted_word == false {
            total += 1;
        }
        
    }

    println!("Part1 {:?}", total);
}

fn repeat_xx(string: &str) -> bool {
    if string.len() < 4 {
        return false;
    }

    let pair = &string[0..2];
    let remain = &string[2..];

    remain.contains(pair) || repeat_xx(&string[1..])
}

fn part2() {

    let input = fs::read_to_string("input.txt").expect("Unable to read input file");
    let words = input.split("\n").collect::<Vec<&str>>();

    let total = words.iter()
                .filter(|word| repeat_xx(word))
                .filter(|z| z.chars().zip(z.chars().skip(2)).any(|(a, b)| a == b))
                .count();

    println!("Part2 {:?}", total);
}

fn main() {
    part1();
    part2();
}
