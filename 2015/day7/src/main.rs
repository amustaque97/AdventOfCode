use std::collections::HashMap;

fn signal<'a>(wire: &'a str, wires: &mut HashMap<&'a str, &'a str>) -> u16 {
        if let Ok(num) = wire.parse::<u16>() {
            // println!("Num: {num}");
            return num;
        }
        let value = wires.get(wire).unwrap();
        if value.contains("AND") {
            let parts: Vec<&str> = value.split(" AND ").collect();
            let l = signal(parts[0].clone(), wires);
            let r = signal(parts[1].clone(), wires);
            let res: &'a str = format!("{}", l & r).as_str();
            wires.insert(wire, res);
            return signal(parts[0], wires) & signal(parts[1], wires);
        }
        if value.contains("OR") {
            let parts: Vec<&str> = value.split(" OR ").collect();
            return signal(parts[0], wires) | signal(parts[1], wires);
        }
        if value.contains("LSHIFT") {
            let parts: Vec<&str> = value.split(" LSHIFT ").collect();
            return signal(parts[0], wires) << signal(parts[1], wires);
        }
        if value.contains("RSHIFT") {
            let parts: Vec<&str> = value.split(" RSHIFT ").collect();
            return signal(parts[0], wires) >> signal(parts[1], wires);
        }
        if value.contains("NOT") {
            let parts: Vec<&str> = value.split("NOT ").collect();
            return !signal(parts[1], wires);
        }

        return signal(value, wires);
}

fn part1() {
    let input = include_str!("../input.txt");
    let mut wires: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let wire = parts[1];
        let value = parts[0];
        wires.insert(wire, value);
    }

    // for (k, v) in &wires {
    //     println!("{} -> {}", v, k);
    // }
    println!("Part1 {}", signal("a", &mut wires));

}

fn main() {
    part1();
}
