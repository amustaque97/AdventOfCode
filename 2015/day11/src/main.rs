fn valid(input: &str) -> bool {
    let mut found_straight = false;
    let mut pairs = 0;

    let mut iter = input.chars();
    let mut prev = iter.next().unwrap();
    let mut pair = prev;

    if matches!(prev, 'i' | 'o' | 'l') {
        return false;
    }

    let mut straight_count = 1;
    for ch in iter {
        if matches!(ch, 'i' | 'o' | 'l') {
            return false;
        }

        if prev as u8 +1 == ch as u8 {
            straight_count += 1;
        } else {
            if straight_count >= 3 {
                found_straight = true;
            }
            straight_count = 1;
        }

        if pair == ch {
           pairs += 1;
           pair = '\0';
        } else {
            pair = ch;
        }

        prev = ch;
    }

    found_straight && pairs >= 2
}

fn incr(input: &str) -> String {
    let mut result = String::from("");
    let mut carry = true;

    for c in input.chars().rev() {
        if carry {
            if c == 'z' {
                result.push('a');
            } else {
                result.push(((c as u8) + 1) as char);
                carry = false;
            }
        } else {
            result.push(c);
        }
    }

    result.chars().rev().collect()
}

fn main() {
    let mut input = String::from("hxbxwxba");
    while !valid(&input) {
        input = incr(&input);
    }

    println!("Part1 {}", input);

    input = incr(&input);
    while !valid(&input) {
        input = incr(&input);
    }

    println!("Part2 {}", input);
}

#[cfg(test)]
mod tests {
    use crate::{incr, valid};

    #[test]
    fn test_hijklmmn() {
        assert!(!valid("hijklmmn"));
    }

    #[test]
    fn test_abbceffg() {
        assert!(!valid("abbceffg"));
    }

    #[test]
    fn test_abbcegjk() {
        assert!(!valid("abbcegjk"));
    }

    #[test]
    fn can_increment_with_carry() {
        assert_eq!(incr("abcdz"), "abcea");
    }
}
