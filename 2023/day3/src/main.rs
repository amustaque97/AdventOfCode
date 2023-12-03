const DR: [i32; 8] = [-1, 1, 0, 0, -1, -1, 1, 1];
const DY: [i32; 8] = [0, 0, -1, 1, -1, 1, -1, 1];
#[derive(Debug, Clone)]
struct Parser {
    buf: String,
    cursor: usize,
    start: i32,
    end: i32,
    line: usize,
    curr_num: String,
}

impl Parser {
    fn new(input: &String, line: usize) -> Parser {
        Parser {
            buf: input.to_string(),
            cursor: 0,
            start: -1,
            end: -1,
            line,
            curr_num: String::new(),
        }
    }

    fn next(&mut self) -> char {
        // println!(
        //     "{} => {}, cursor {}, len {}",
        //     self.line,
        //     self.buf,
        //     self.cursor,
        //     self.buf.len()
        // );
        let ch = self.buf.chars().nth(self.cursor).unwrap();
        if self.cursor < self.buf.len() {
            self.cursor += 1;
        }
        ch
    }

    pub fn next_number(&mut self) -> String {
        let mut command = String::new();
        if self.cursor >= self.buf.len() {
            return command;
        }
        let mut ch = self.next();
        let start = if ch.is_digit(10) {
            self.cursor as i32
        } else {
            -1
        };
        while ch.is_digit(10) {
            if start != self.start && self.start < self.cursor as i32 {
                self.start = self.cursor as i32;
            }
            command.push(ch);
            if self.cursor < self.buf.len() {
                ch = self.next();
            } else {
                break;
            }
        }
        if self.end < self.cursor as i32 {
            self.end = self.cursor as i32;
        }
        self.curr_num = command.clone();
        command
    }
}

// return true if any adjacent direction contains symbol
// else false
fn check_all_directions(parser: &Parser, grid: &Vec<Vec<char>>, idx: usize) -> bool {
    let mut symbol = false;
    let start = (parser.start - 1) as usize;
    let end = (parser.end - 1) as usize;
    let n = grid.len() as i32;

    // downward
    for i in start..end {
        for j in 0..DR.len() {
            let new_x: i32 = idx as i32 + DR[j];
            let new_y: i32 = i as i32 + DY[j];
            if new_x >= n || new_x <= 0 || new_y >= n || new_y <= 0 {
                // dbg!(&parser.curr_num, "~");
                continue;
            }
            let ch = grid[new_x as usize][new_y as usize];
            // println!("{} => {}", parser.curr_num, ch);
            if !ch.is_digit(10) && ch != '.' {
                symbol = true;
                break;
            }
        }
    }
    return symbol;
}

fn main() {
    let lines = include_str!("../input.txt")
        .split('\n')
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let matrix: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut total = 0;
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut parser = Parser::new(&line.to_string(), idx);
        while !parser.next_number().is_empty() || parser.cursor < parser.buf.len() {
            if parser.curr_num.is_empty() {
                continue;
            } else if check_all_directions(&parser, &matrix, idx) {
                // println!("{}", parser.curr_num);
                total += parser.curr_num.parse::<usize>().unwrap_or(0);
            }
        }
    }
    println!("{:?}", total);

    // part2
    let mut total2 = 0;
    let mut numbers: Vec<(Parser, i32)> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut parser = Parser::new(&line.to_string(), idx);
        while !parser.next_number().is_empty() || parser.cursor < parser.buf.len() {
            if parser.curr_num.is_empty() {
                continue;
            } else {
                // println!("{}", parser.curr_num);
                numbers.push((parser.clone(), idx as i32));
            }
        }
    }

    let n = matrix.len();
    let m = matrix[0].len();
    // println!("{}, {}", n, m);

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '*' {
                    total2 += get_part_numbers(i as i32, j as i32, &numbers, &matrix);
            }
        }
    }

    println!("{:?}", total2);
}

fn get_part_numbers(i: i32, j: i32, numbers: &Vec<(Parser, i32)>, grid: &Vec<Vec<char>>) -> u32 {
    let mut visited: Vec<String> = Vec::new();
    let mut gears: Vec<String> = Vec::with_capacity(2);
    let n = grid.len() as i32;
    for k in 0..8 {
        let new_x = i as i32 + DR[k];
        let new_y = j as i32 + DY[k];
        if new_x >= n || new_x < 0 || new_y >= n || new_y < 0 {
            // dbg!(&parser.curr_num, "~");
            continue;
        }

        for (num, row) in numbers {
            for y in num.start - 1..num.end - 1 {
                let t = format!("{}::{}:{}", num.curr_num, num.start, num.end);
                if new_x == *row && new_y == y && !visited.contains(&t) {
                    visited.push(t);
                    if gears.len() == 2{
                        return 0;
                    }
                    gears.push(num.curr_num.clone());
                    // println!(
                    //     "num {}\tnew_x {}\tnew_y {}\trow {}\tcol {}",
                    //     num.curr_num, new_x, new_y, *row, y
                    // );
                }
            }
        }
    }

    if gears.len() < 2 {
        return 0;
    }

    gears[0].parse::<u32>().unwrap() * gears[1].parse::<u32>().unwrap()
}
