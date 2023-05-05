static mut MIN_LEN: i32 = 9999;
static mut RESULT: Vec<Vec<usize>> = Vec::new();
fn find_combination(idx: usize, target: usize, c: Vec<usize>, mut tmp: Vec<usize>) -> usize {
    if target == 0 {
        unsafe {
            MIN_LEN = MIN_LEN.min(tmp.len() as i32);
            RESULT.push(tmp.clone());
        }
        return 1;
    } else if idx >= c.len() {
        return 0;
    }

    let mut x = 0;
    let mut y = 0;
    if c[idx] <= target {
        let t = c[idx];
        tmp.push(t);
        x += find_combination(idx + 1, target - c[idx], c.clone(), tmp.clone());
        tmp.pop();
    }
    y += find_combination(idx + 1, target, c, tmp);
    x + y
}
fn part1() {
    // let containers = vec![20, 15, 10, 5, 5];
    let containers = vec![
        50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40,
    ];

    let tmp: Vec<usize> = Vec::new();

    let ans = find_combination(0, 150, containers, tmp);

    println!("Part1 {}", ans);
    unsafe {
        println!("{:?}", RESULT);
    }
}

fn part2() {
    let mut res = 0;
    unsafe{
        for t in &RESULT {
            if t.len() == MIN_LEN as usize {
                res += 1;
            }
        }
    }
    println!("Part2 {}", res);
}

fn main() {
    part1();
    part2();
}

