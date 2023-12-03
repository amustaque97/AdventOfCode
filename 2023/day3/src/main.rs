#[derive(Debug)]
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

    let dr: [i32; 8] = [-1, 1, 0, 0, -1, -1, 1, 1];
    let dy: [i32; 8] = [0, 0, -1, 1, -1, 1, -1, 1];

    // downward
    for i in start..end {
        for j in 0..dr.len() {
            let new_x: i32 = idx as i32 + dr[j];
            let new_y: i32 = i as i32 + dy[j];
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
}
