use serde_json::Value;

fn calc(input: &Value) -> i64 {
    
    match input {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|x| calc(x)).sum(),
        Value::Object(o) => o.values().map(|entry| calc(entry)).sum(),
    }
}
fn calc_without_red(input: &Value) -> i64 {
    
    match input {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|x| calc_without_red(x)).sum(),
        Value::Object(o) => {
            for (_, v) in o.iter() {
                if v == "red" {
                   return 0; 
                }
            }
            return o.values().map(|entry| calc_without_red(entry)).sum();
        },
    }
}

fn part1() {

    let input = include_str!("../input.txt");
    let v: Value = serde_json::from_str(input).unwrap();
    println!("Part1 {}" ,calc(&v));
}

fn part2() {

    let input = include_str!("../input.txt");
    let v: Value = serde_json::from_str(input).unwrap();
    println!("Part2 {}" ,calc_without_red(&v));
}

fn main() {
    part1();
    part2();
}
