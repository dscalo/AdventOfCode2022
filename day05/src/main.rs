extern crate file_reader;
use file_reader::read_file;

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new_from_vec(v: &Vec<usize>) -> Instruction {
        return Instruction {
            amount: v[0],
            // 0 based array so i don't get confused
            from: v[1] -1,
            to: v[2] -1 ,
        };
    }
}

type Instructions = Vec<Instruction>;
type Stacks = Vec<Vec<char>>;

fn parse_items(s: &str, items: &mut Instructions) {
    let line = s.parse::<String>().unwrap();
    let instr = line.split(",").map(|n| n.parse::<usize>().unwrap()).collect();

    items.push(Instruction::new_from_vec(&instr))
}

enum Input {
    Test,
    Puzzle,
}

fn get_stacks(input: Input) -> Stacks {
    match input {
        Input::Test => return vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        Input::Puzzle => {
            return Stacks::new();
        }
    }
}

fn pretty_print(stacks: &Stacks) {
    println!("");
    let mut largest = 0;  

    for s in stacks {
        if s.len() > largest {
            largest = s.len();
        }
    }

   let mut idx = largest;

    while idx > 0 {
        idx -= 1;
        for s in stacks {
            if idx >= s.len() {
                print!("     ");
                continue;
            }
            print!(" [{}] ", s[idx]);
        }
        println!("");
        
    }

    
    for i in 0..stacks.len() {
        print!("  {}  ",i+1);
    }

    println!("\n----------------------------------\n");
}


fn part1(stacks: &mut Stacks, instructions: &Instructions) {
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let c = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(c);
        }
    }
}

fn part2(stacks: &mut Stacks, instructions: &Instructions) {
    let mut queue = Vec::new();

    for instruction in instructions {
        for _ in 0..instruction.amount {
            let c = stacks[instruction.from].pop().unwrap();
            queue.push(c);
           
        }

        while queue.len() > 0 {
            let q = queue.pop().unwrap();
            stacks[instruction.to].push(q);
        }
    }
}

fn main() {
    let mut instructions = Instructions::new();

    read_file("test01.txt", parse_items, &mut instructions);

    let mut stacks_p1 = get_stacks(Input::Test);
    println!("PART 1");
    pretty_print(&stacks_p1);
    part1(&mut stacks_p1, &instructions);
    pretty_print(&stacks_p1);
   

    let mut stacks_p2 = get_stacks(Input::Test);
    println!("\n\nPART 2");
    pretty_print(&stacks_p2);
    part2(&mut stacks_p2, &instructions);
    pretty_print(&stacks_p2);
   

 
}
