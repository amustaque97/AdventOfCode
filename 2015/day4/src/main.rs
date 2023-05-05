use md5;


fn part1() {
    let input = "ckczppom";

    for i in 0..std::u32::MAX {
        let potential = format!("{}{}", input, i);
        let digest = md5::compute(potential);
        let digest_str = format!("{:x}", digest);
        if digest_str.starts_with("00000") {
            println!("Part1 {}: {}", i, digest_str);
            break;
        }

    }
}

fn part2() {
    let input = "ckczppom";

    for i in 0..std::u32::MAX {
        let potential = format!("{}{}", input, i);
        let digest = md5::compute(potential);
        let digest_str = format!("{:x}", digest);
        if digest_str.starts_with("000000") {
            println!("Part2 {}: {}", i, digest_str);
            break;
        }

    }
}
fn main() {
    part1();
    part2();
}
