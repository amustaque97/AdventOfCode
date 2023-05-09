use std::collections::HashSet;

#[derive(Debug)]
struct Code {
    chr: String,
    val: String,
}

fn part1(instructions: Vec<Code>, mol: String) {
    let chars: Vec<char> = mol.chars().collect();
    let mut set: HashSet<String> = HashSet::new();

    for i in 0..chars.len() {
        // println!("{}", chars[i]);
        for inst in &instructions {
            let c = inst;
            // println!("{} == {} = {}", &c.chr, chars[i].to_string(), c.chr == String::from(chars[i]));
            let mut len = c.chr.len();
            if len != 1 && i+len >= mol.len() {
                len = 0;
            }
            if c.chr == chars[i..(i+len)].iter().collect::<String>() {
                let t: String =
                    chars.get(0..i).unwrap().iter().collect::<String>() + &c.val + &chars.get(i+len..).unwrap().iter().collect::<String>();
                set.insert(t);
            }
        }
    }

    println!("Part1 {}", set.len());
}

fn main() {
    let mut instructions: Vec<Code> = Default::default();
    let mut mol: String = String::new();
    let input = include_str!("../input.txt");

    for line in input.lines() {
        if line.contains("=>") {
            let arr: Vec<&str> = line.split("=>").map(|s| s.trim()).collect();
            let c = Code {
                chr: arr[0].to_string(),
                val: arr[1].to_string(),
            };
            instructions.push(c);
        } else if !line.is_empty() && !line.contains("=>") {
            mol = line.to_string();
        }
    }

    // for c in instructions {
    //     println!("{:?}", c);
    // }
    //
    // println!("{}", mol);
    //
    part1(instructions, mol);
}
