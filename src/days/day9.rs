use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use crate::days::day2::ParseError;

pub fn positions(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut rope = Rope::new();
    for (line, s) in input.split("\n").enumerate() {
        if s.trim() == "" {
            continue
        }
        let mv: Move = match s.parse() {
            Ok(m) => m,
            Err(e) => return format!("Invalid move: {} - {}. {}", line, s, e)
        };
        //println!("{}", s);
        rope.do_move(mv);
    }

    return rope.tail_positions.len().to_string();
}

pub fn positions1(path: &str) -> String {
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return format!("Fail - {}", e)
    };
    let mut rope = Rope1::new();
    for (line, s) in input.split("\n").enumerate() {
        if s.trim() == "" {
            continue
        }
        let mv: Move = match s.parse() {
            Ok(m) => m,
            Err(e) => return format!("Invalid move: {} - {}. {}", line, s, e)
        };
        rope.do_move(mv);
    }

    return (rope.tail_positions.len() + 1).to_string();
}

struct Rope {
    head: [i32; 2],
    tail: [i32; 2],
    tail_positions: HashSet<[i32; 2]>
}

impl Rope {
    pub fn new() -> Rope {
        return Rope{
            head: [0, 0],
            tail: [0, 0],
            tail_positions: HashSet::new()
        }
    }
    pub fn do_move(&mut self, mv: Move) {
        let (indx, st, incr) = match mv {
            Move::Up(st) => (0, st, 1),
            Move::Down(st) => (0, st, -1),
            Move::Left(st) => (1, st, -1),
            Move::Right(st) => (1, st, 1)
        };
        for _ in 0..st {
            self.head[indx] += incr as i32;
            self.ensure_tail(indx, incr);
        }
    }
    fn ensure_tail(&mut self, indx: usize, incr: i8) {
        if ((self.head[0] - self.tail[0]).abs() > 1)  || ((self.head[1] - self.tail[1]).abs() > 1) {
            self.tail[1 - indx] = self.head[1 - indx];
            self.tail[indx] = self.head[indx] - incr as i32;
            self.tail_positions.insert(self.tail);
        }
    }
}

struct Rope1 {
    head: [i32; 2],
    tails: Vec<[i32; 2]>,
    tail_positions: HashSet<[i32; 2]>
}

impl Rope1 {
    pub fn new() -> Rope1 {
        return Rope1{
            head: [0, 0],
            tails: vec![[0, 0]; 9],
            tail_positions: HashSet::new()
        }
    }
    pub fn do_move(&mut self, mv: Move) {
        let (indx, st, incr) = match mv {
            Move::Up(st) => (0, st, -1),
            Move::Down(st) => (0, st, 1),
            Move::Left(st) => (1, st, -1),
            Move::Right(st) => (1, st, 1)
        };
        for _ in 0..st {
            self.head[indx] += incr as i32;
            self.ensure_tail(indx);
        }
    }
    fn ensure_tail(&mut self, indx: usize) {
        let tail_pos = self.tails.len() - 1;
        for i in 0..self.tails.len() {
            let head: [i32; 2];
            if i as i32 - 1 < 0 {
                head = self.head
            }  else {
                head = self.tails[i - 1]
            }
            if ((head[0] - self.tails[i][0]).abs() > 1) || ((head[1] - self.tails[i][1]).abs() > 1) {
                if (head[0] != self.tails[i][0]) && (head[1] != self.tails[i][1]) {
                    if head[1 - indx] > self.tails[i][1 - indx] {
                        self.tails[i][1 - indx] += 1
                    } else {
                        self.tails[i][1 - indx] -= 1
                    }
                    if head[indx] > self.tails[i][indx] {
                        self.tails[i][indx] += 1
                    } else {
                        self.tails[i][indx] -= 1
                    }
                } else {
                    let mut tindx: usize = 0;
                    if head[1] != self.tails[i][1] {
                        tindx = 1
                    }
                    if head[tindx] > self.tails[i][tindx] {
                        self.tails[i][tindx] += 1
                    } else {
                        self.tails[i][tindx] -= 1
                    }
                }
                if i == tail_pos {
                    self.tail_positions.insert(self.tails[tail_pos]);
                }
            }
        }
    }
    /*fn print(&self) {
        let delta = 12;
        let x = self.head[0] - delta;
        let y = self.head[1] - delta;
        print!("\t");
        for j in y..y+delta * 2 {
            print!("{}\t", j)
        }
        println!("");
        for i in x..x+delta * 2 {
            print!("{}\t", i);
            for j in y..y+delta * 2 {
                if i == self.head[0] && j == self.head[1] {
                    print!("H\t");
                    continue
                }
                let mut found = false;
                for (it, t) in self.tails.iter().enumerate() {
                    if i == t[0] && j == t[1] {
                        print!("{}\t", it + 1);
                        found = true;
                        break
                    }
                }
                if found {
                    continue
                }
                print!(".\t")
            }
            println!("")
        }
        println!("")
    }*/
}

enum Move {
    Left(i8),
    Right(i8),
    Up(i8),
    Down(i8)
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(ParseError{msg: format!("Expect 2 parts: {}", s)})
        }
        let steps: i8 = match parts[1].parse() {
            Ok(st) => st,
            Err(e) => return Err(ParseError{msg: format!("Invalid number of steps: {}. {}", parts[1], e)})
        };
        let mv =  match parts[0].to_uppercase().as_str() {
            "U" => Move::Up(steps),
            "D" => Move::Down(steps),
            "L" => Move::Left(steps),
            "R" => Move::Right(steps),
            _ => return Err(ParseError{msg: format!("Unsupported direction: {}", parts[0])})
        };

        return Ok(mv)
    }
}