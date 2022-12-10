use std::collections::VecDeque;
extern crate file_reader;
use file_reader::read_file;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    noop,
    addx(i64),
    exe(i64),
}

type Instructions = Vec<Instruction>;
type Queues = Vec<Vec<char>>;
type Line = Vec<char>;

fn parse_items(s: &str, items: &mut Instructions) {
    let line = s.parse::<String>().unwrap();

    let row: Vec<String> = line.split_whitespace().map(str::to_string).collect();

    if row[0] == "addx".to_string() {
        let amount = row[1].parse::<i64>().unwrap();
        items.push(Instruction::addx(amount))
    } else {
        items.push(Instruction::noop);
    }

    //items.push(Mv::new(dir, amount));
}

fn part1(instructions: &Instructions) -> i64 {
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut X = 1;
    let mut signal_str = 0;
    let mut idx = 0;

    let mut cur_instr: Option<Instruction> = None;

    for c in 1..=220 {
        match cur_instr {
            None => {
                if idx < instructions.len() {
                    cur_instr = Some(instructions[idx]);
                    idx += 1;
                }
            }
            _ => {}
        }

        //println!("cycle:{}, X: {} cur_instr: {:?}", c,X, cur_instr);
        if cycles.contains(&c) {
            signal_str += c * X;
        }

        match cur_instr.unwrap() {
            Instruction::noop => {
                cur_instr = None;
            }
            Instruction::addx(val) => {
                cur_instr = Some(Instruction::exe(val));
            }
            Instruction::exe(val) => {
                X += val;
                cur_instr = None
            }
        }
    }

    signal_str
}

fn pretty_print(queues: &Queues) {
    for line in queues {
        for c in line {
            print!("{}", c);
        }
        print!("\n");
    }
    print!("\n\n");
}

fn part2(instructions: &Instructions) {
    let cycles = vec![40, 80, 120, 160, 200, 240];
    let mut X = 1;
    let mut idx = 0;

    let mut queues = Queues::new();
    let mut line = Line::new();

    let mut cur_instr: Option<Instruction> = None;
    let mut md = 0;

    for c in 1..=240 {
        let mut add_to_x = 0;
        match cur_instr {
            None => {
                if idx < instructions.len() {
                    cur_instr = Some(instructions[idx]);
                    idx += 1;
                }
            }
            _ => {}
        }

        match cur_instr.unwrap() {
            Instruction::noop => {
                cur_instr = None;
            }
            Instruction::addx(val) => {
                cur_instr = Some(Instruction::exe(val));
            }
            Instruction::exe(val) => {
                add_to_x = val;
                cur_instr = None
            }
        }

        println!("During cycle: {} | X: {}", c, X);
        if (c - md - 1) as i64 == X
            || (c - md - 1) as i64 == (X - 1)
            || (c - md - 1) as i64 == (X + 1)
        {
            line.push('#');
        } else {
            line.push('.');
        }
        if cycles.contains(&c) {
            queues.push(line);
            line = Line::new();
            md += 40;
        }

        X += add_to_x;
        println!("cycle:{}, X: {} cur_instr: {:?}", c, X, cur_instr);
    }
    pretty_print(&queues);
}

fn main() {
    let mut instructions = Instructions::new();

    read_file("puzzle.txt", parse_items, &mut instructions);

    //println!("{:?}", instructions);

    let p1 = part1(&instructions);
    println!("Part 1: {}", p1);

    part2(&instructions);
}
