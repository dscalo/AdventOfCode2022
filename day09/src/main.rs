extern crate file_reader;
use file_reader::read_file;
use std::cmp;
use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<u32>>;
type Moves = Vec<Mv>;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
    fn distance_to(&self, p2: &Point) -> i64 {
        // |x1 - x2| + |y1 - y2|
        i64::abs(self.x - p2.x) + i64::abs(self.y - p2.y)
    }
    fn is_adjacent_to(&self, p2: &Point) -> bool {
        if self.x == p2.x && self.y == p2.y {
            return true;
        }

        if self.x + 1 == p2.x && self.y == p2.y {
            return true;
        }

        if self.x - 1 == p2.x && self.y == p2.y {
            return true;
        }

        if self.x == p2.x && self.y + 1 == p2.y {
            return true;
        }

        if self.x == p2.x && self.y - 1 == p2.y {
            return true;
        }

        // dia
        if self.x + 1 == p2.x && self.y + 1 == p2.y {
            return true;
        }

        if self.x - 1 == p2.x && self.y + 1 == p2.y {
            return true;
        }

        if self.x + 1 == p2.x && self.y - 1 == p2.y {
            return true;
        }

        if self.x - 1 == p2.x && self.y - 1 == p2.y {
            return true;
        }

        false
    }
}

#[derive(Debug)]
struct Mv {
    dir: char,
    amount: i64,
}

impl Mv {
    fn new(dir: char, amount: i64) -> Mv {
        Mv { dir, amount }
    }
}

fn parse_items(s: &str, items: &mut Moves) {
    let line = s.parse::<String>().unwrap();

    let row: Vec<String> = line.split_whitespace().map(str::to_string).collect();

    let dir = row[0].chars().nth(0).unwrap();
    let amount = row[1].parse::<i64>().unwrap();

    items.push(Mv::new(dir, amount));
}

fn get_spaces(head: &Point, tail: &Point, visited: &mut HashSet<Point>) {
    // horizontal
    if head.y == tail.y {
        for x in cmp::min(head.x, tail.x) + 1..cmp::max(head.x, tail.x) {
            visited.insert(Point::new(x, head.y));
        }
        return;
    }

    // vertical
    if head.x == tail.x {
        for y in cmp::min(head.y, tail.y) + 1..cmp::max(head.y, tail.y) {
            visited.insert(Point::new(head.x, y));
        }
        return;
    }

    // diagonal
    if i64::abs(head.y - tail.y) == 1 {
        let temp = Point::new(tail.x, head.y);
        //visited.insert(temp.clone());
        get_spaces(head, &temp, visited);
        return;
    }

    if i64::abs(head.x - tail.x) == 1 {
        let temp = Point::new(head.x, tail.y);
        //visited.insert(temp.clone());
        get_spaces(head, &temp, visited);
    }
}

fn print_h_t(head: &Point, tail: &Point) {
    println!("Head: {},{} Tail: {},{}", head.x, head.y, tail.x, tail.y);
}

fn part1(moves: &Moves) -> i64 {
    let mut head = Point::new(0, 0);
    let mut tail = Point::new(0, 0);
    let mut visited = HashSet::new();

    visited.insert(tail.clone());

    let mut ct = 0;

    for mv in moves {
        ct += 1;

        match mv.dir {
            'U' => head.y += mv.amount,
            'R' => head.x += mv.amount,
            'D' => head.y -= mv.amount,
            'L' => head.x -= mv.amount,
            _ => panic!("Invalid move"),
        }

        if !tail.is_adjacent_to(&head) {
            get_spaces(&head, &tail, &mut visited);
            // move the tail
            match mv.dir {
                'U' => {
                    tail.y = head.y - 1;
                    tail.x = head.x;
                }
                'R' => {
                    tail.y = head.y;
                    tail.x = head.x - 1;
                }
                'D' => {
                    tail.y = head.y + 1;
                    tail.x = head.x;
                }
                'L' => {
                    tail.y = head.y;
                    tail.x = head.x + 1;
                }
                _ => panic!("Invalid move"),
            }
        }
        //print_h_t(&head, &tail);

        // println!(
        //     "{}:{}{} - head: {},{} - tail: {},{}",
        //     ct, mv.dir, mv.amount, head.x, head.y, tail.x, tail.y
        // );
    }

    // for p in &visited {
    //     println!("({},{})", p.x, p.y);
    // }

    visited.len() as i64
}

fn part2(moves: &Moves) -> i64 {
    let mut head = Point::new(0, 0);
    let mut tails = Vec::new();

    for _ in 0..9 {
        tails.push(Point::new(0, 0));
    }
    let mut visited = HashSet::new();

    visited.insert(head.clone());

    let mut ct = 0;

    for mv in moves {
        ct += 1;
        for a in 0..mv.amount {
            //println!("amount : {}", a);
            match mv.dir {
                'U' => head.y += 1,
                'R' => head.x += 1,
                'D' => head.y -= 1,
                'L' => head.x -= 1,
                _ => panic!("Invalid move"),
            }
            //println!("HEAD: {},{}", head.x, head.y);
            let mut t_head = head.clone();
            for i in 0..tails.len() {
                
                if !tails[i].is_adjacent_to(&t_head) {
                    //println!("tails[i]: {:?} | t_head: {:?}", tails[i],t_head);
                    // if i == 8 {
                    //    // print_h_t(&t_head, &tails[i]);
                    //     get_spaces(&t_head, &tails[i], &mut visited);
                    // }
                    // move the tail
                    if tails[i].x == t_head.x && i64::abs(t_head.y - tails[i].y) == 2 {
                        if t_head.y - tails[i].y < 0 {
                            tails[i].y -= 1;
                        } else {
                            tails[i].y += 1;
                        }
                     
                    }

                    if tails[i].y == t_head.y && i64::abs(t_head.x - tails[i].x) == 2 {
                        if t_head.x - tails[i].x < 0 {
                            tails[i].x -= 1;
                        } else {
                            tails[i].x += 1;
                        }
                       
                    }

                    if tails[i].x != t_head.x && tails[i].y != t_head.y {
                        if t_head.y - tails[i].y < 0 {
                            tails[i].y -= 1;
                        } else {
                            tails[i].y += 1;
                        }

                        if t_head.x - tails[i].x < 0 {
                            tails[i].x -= 1;
                        } else {
                            tails[i].x += 1;
                        }
                    }

                    if i == 8 {
                        visited.insert(tails[i].clone());
                    }
                }
                t_head = tails[i].clone();
            }
          
        }
       //println!("--------------------------");
    }

    //println!("tails final position");
    for p in &tails {
        //println!("({},{})", p.x, p.y);
    }

    visited.len() as i64
}

fn main() {
    let mut moves = Moves::new();
    read_file("puzzle.txt", parse_items, &mut moves);

    // let mut hs = HashSet::new();

    // let head = Point::new(5, 3);
    // let tail = Point::new(5, 3);
    //get_spaces(&head, &tail, &mut hs);

    //println!("{:?}", head.is_adjacent_to(&tail));

    // 6264 is too low, 8294 is too high
    // part 2 2383 is too low

    let p1 = part1(&moves);
    let p2 = part2(&moves);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
