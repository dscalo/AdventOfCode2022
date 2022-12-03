extern crate file_reader;
use file_reader::read_file;

use std::collections::HashMap;


fn parse_items(s: &str, items: &mut Vec<String>) {
    let line = s.parse::<String>().unwrap();    
    items.push(line)

}

fn get_priority(c: char) -> i64 {
    let mut val = (c.to_ascii_uppercase() as i64) - 64;

    if c.is_uppercase() {
        val += 26;
    }

    val
}


fn part1(items: &Vec<String>) -> i64 {
    let mut tot = 0;    

    for item in items {
        let mut left = HashMap::new();
        let mut right = HashMap::new();

        let cs: Vec<char> = item.chars().collect();

        let ln = cs.len() / 2 as usize;

        for i in 0..ln {
          if right.contains_key(&cs[i]) {
            tot += get_priority(cs[i]);
            break
          }

          left.insert(cs[i].clone(), 1);

          if left.contains_key(&cs[i+ln]) {
            tot += get_priority(cs[i+ln]);
            break
          }

          right.insert(cs[i+ ln].clone(), 1);
        }
    }

    tot
}

fn part2(items: &Vec<String>) -> i64 {
    let mut tot = 0;
    let mut i = 0;

    while i < items.len() {
        let mut tracker = HashMap::new();

        let mut cs: Vec<char> = items[i].chars().collect();

        for c in cs {
            tracker.insert(c.clone(), 1);
        }

        cs = items[i+1].chars().collect();

        for c in cs {
            if tracker.contains_key(&c) {
                tracker.insert(c.clone(), 2);
            }
        }

        cs = items[i+2].chars().collect();

        for c in cs {
           if let Some(n) = tracker.get(&c) {
            if n == &2 {
                tot += get_priority(c);
                break;
            }
           }
        }

        i += 3;

    }    

    tot
}


fn main() {
    let mut items: Vec<String> = Vec::new();
    read_file("puzzle.txt", parse_items, &mut items);

    let p1 = part1(&items);
    let p2 = part2(&items);


    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
