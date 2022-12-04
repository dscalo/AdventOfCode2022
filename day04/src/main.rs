extern crate file_reader;
use file_reader::read_file;

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

type Assignments = Vec<Pair>;

impl Assignment {
    fn new_from_vec(v: &Vec<u32>) -> Assignment {
        Assignment {
            start: v[0],
            end: v[1],
        }
    }
}

#[derive(Debug)]
struct Pair(Assignment, Assignment);

// 2-4,6-8
fn parse_items(s: &str, items: &mut Assignments) {
    let line = s.parse::<String>().unwrap();
    let pairs: Vec<&str> = line.split(",").collect();

    let a1 = Assignment::new_from_vec(
        &pairs[0]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .collect(),
    );
    let a2 = Assignment::new_from_vec(
        &pairs[1]
            .split("-")
            .map(|n| n.parse::<u32>().unwrap())
            .collect(),
    );

    items.push(Pair(a1, a2));
}

fn is_contained(a1: &Assignment, a2: &Assignment) -> bool {
    if a1.start <= a2.start && a1.end >= a2.end {
        return true
    }

    false
}

/*
    5-7 7-9
    5,6,7
        7,8,9

    2-8 3,7
    2,3,4,5,6,7,8
      3,4,5,6,7
*/
fn is_overlapping(a1: &Assignment, a2: &Assignment) -> bool {
    if a2.start >= a1.start && a2.start <= a1.end {
        return true
    }

    false
}

fn part1(assignments: &Assignments) -> u32 {
    let mut tot = 0;

    for pair in assignments {
        if is_contained(&pair.0, &pair.1) {
            tot += 1;
            continue;
        }

        if is_contained(&pair.1, &pair.0) {
            tot += 1;
        }
    }

    tot
}

fn part2(assignments: &Assignments) -> u32 {
    let mut tot = 0;

    for pair in assignments {
        if is_overlapping(&pair.0, &pair.1) {
            tot += 1;
            continue;
        }

        if is_overlapping(&pair.1, &pair.0) {
            tot += 1;
        }
    }

    tot
}


fn main() {
    let mut assignments = Assignments::new();
    read_file("puzzle.txt", parse_items, &mut assignments);
   

    let p1 = part1(&assignments);
    let p2 = part2(&assignments);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
