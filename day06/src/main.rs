extern crate file_reader;
use file_reader::read_file;
use std::collections::{HashSet, VecDeque};

type Chars = Vec<char>;

fn parse_items(s: &str, items: &mut Chars) {
    let line = s.parse::<String>().unwrap();
    let cs: Vec<char> = line.chars().collect();

    for c in cs {
        items.push(c);
    }
}

fn get_test_data(n: usize, part: usize) -> Chars {
    if part == 1 {
        match n {
            0 => return "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(),
            1 => return "nppdvjthqldpwncqszvftbrmjlhg".chars().collect(),
            2 => return "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(),
            3 => return "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(),
            _ => return Vec::new(),
        }
    } else {
        match n {
            0 => return "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect(),
            1 => return "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(),
            2 => return "nppdvjthqldpwncqszvftbrmjlhg".chars().collect(),
            3 => return "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(),
            4 => return "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(),
            _ => return Vec::new(),
        }
    }
}

fn is_unique(dq: &VecDeque<char>, size: usize) -> bool {
    let hs = dq.iter().cloned().collect::<HashSet<char>>();

    hs.len() == size
}

fn solve(chars: &Chars, size: usize) -> usize {
    let mut dq = VecDeque::new();

    let mut end = size;

    for i in 0..size {
        dq.push_back(chars[i])
    }

    if is_unique(&dq, size) {
        return end + 1;
    }

    while end < chars.len() {
        dq.pop_front();

        dq.push_back(chars[end].clone());

        if is_unique(&dq, size) {
            break;
        }

        end += 1;
    }

    end + 1
}


fn main() {
    let mut puzzle = Chars::new();
    read_file("puzzle.txt", parse_items, &mut puzzle);

    // Part 1 
    for i in 0..4 {
        let data = get_test_data(i, 1);
        let ans = solve(&data, 4);

        println!("Part 1 (Test #: {}): {}", i, ans);
    }

    let ans = solve(&puzzle, 4);

    println!("Part 1 (Puzzle): {}", ans);

    // Part 2
    for i in 0..5 {
        let data = get_test_data(i, 2);
        let ans = solve(&data, 14);

        println!("Part 2 (Test #: {}): {}", i, ans);
    }

    let ans = solve(&puzzle, 14);

    println!("Part 2 (Puzzle): {}", ans);

}
