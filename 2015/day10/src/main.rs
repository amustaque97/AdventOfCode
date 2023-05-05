fn part1(input: &str) -> String {
    let mut ans = String::new();

    let char_list = input.chars().collect::<Vec<char>>();

    // println!("{:?}", char_list);
    
    let mut index = 0;
    while index < char_list.len() {
        let mut count = 1;
        while index+1 < char_list.len() && char_list[index] == char_list[index+1] {
            count += 1;
            index += 1;
        }
        ans += format!("{}{}", count, char_list[index]).as_str();
        // println!("{} => {}", char_list[index], count);
        index += 1;
    }

    ans

}

fn main() {
    let mut input = String::from("1321131112");
    for _ in 0..40 {
        input = part1(&input);
    }
    // println!("{}", input);
    println!("Part1 {}", input.len());
    for _ in 0..10 {
        input = part1(&input);
    }
    // println!("{}", input);
    println!("Part2 {}", input.len());
}

#[cfg(test)]
mod tests {
    use crate::{main, part1};

    #[test]
    fn call_main() {
        main()
    }

    #[test]
    fn copy_one() {
        assert_eq!(part1("1"), "11".to_string());
    }

    #[test]
    fn copy_input() {
        assert_eq!(part1("1321131112"), "11131221133112".to_string());
    }
}

