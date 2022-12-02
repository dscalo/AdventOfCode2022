extern crate file_reader;
use file_reader::read_file;

type Calories = Vec<Option<i64>>;

fn parse_items(s: &str, items: &mut Calories) {
    let num = s.parse::<i64>();    

    match num {
        Ok(n) => items.push(Some(n)),
        Err(_) => items.push(None)
    }
}


fn part1(calories: &Calories) -> i64 {
    let mut tot = i64::MIN;
    let mut elf_tot = 0;
    
    for c in calories {
        match c {
            Some(n) => elf_tot += n,
            None => {
                if elf_tot > tot {
                    tot = elf_tot;
                }
                elf_tot = 0;
            }
        }
    }

    tot
}


fn part2(calories: &Calories) -> i64 {
    let mut tots = Vec::new();
    let mut elf_tot = 0;
    let mut tot = 0;
    
    for c in calories {
        match c {
            Some(n) => elf_tot += n,
            None => {
                tots.push(elf_tot);
                elf_tot = 0;
            }
        }
    }

    tots.push(elf_tot);

    tots.sort();   
   

    for i in 1..4{
        tot += tots[tots.len() - i];
    }

    tot
}



fn main() {
    let mut calories = Vec::new();

    read_file("puzzle.txt", parse_items, &mut calories);

    let p1 = part1(&calories);
    let p2 = part2(&calories);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
