extern crate file_reader;
use file_reader::read_file;
use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(y: usize, x: usize) -> Point {
        Point { x, y }
    }
}

fn get_lookup() -> HashMap<char, i64> {
    let mut lookup = HashMap::new();
    lookup.insert('a', 1);
    lookup.insert('b', 2);
    lookup.insert('c', 3);
    lookup.insert('d', 4);
    lookup.insert('e', 5);
    lookup.insert('f', 6);
    lookup.insert('g', 7);
    lookup.insert('h', 8);
    lookup.insert('i', 9);
    lookup.insert('j', 10);
    lookup.insert('k', 11);
    lookup.insert('l', 12);
    lookup.insert('m', 13);
    lookup.insert('n', 14);
    lookup.insert('o', 15);
    lookup.insert('p', 16);
    lookup.insert('q', 17);
    lookup.insert('r', 18);
    lookup.insert('s', 19);
    lookup.insert('t', 20);
    lookup.insert('u', 21);
    lookup.insert('v', 22);
    lookup.insert('w', 23);
    lookup.insert('x', 24);
    lookup.insert('y', 25);
    lookup.insert('z', 26);
    lookup.insert('S', 1);
    lookup.insert('E', 26);

    lookup
}

fn pretty_print(grid: &Grid) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        print!("\n")
    }

    print!("\n\n");
}

fn parse_items(s: &str, items: &mut Grid) {
    let line = s.parse::<String>().unwrap();

    let row: Vec<char> = line.chars().collect();

    items.push(row);
}

fn get_starting_point(grid: &Grid, c: char) -> Point {
    let mut point = Point::new(0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == c {
                point.y = y;
                point.x = x;
            }
        }
    }

    point
}

fn contains_point(points: &Vec<Point>, point: &Point) -> bool {
    let mut ans = false;
    for p in points {
        if point.y == p.y && point.x == p.x {
            ans = true;
            break;
        }
    }

    ans
}

fn part1(grid: &Grid) -> i64 {
    let mut tot = 0;
    let lookup = get_lookup();
    let start = get_starting_point(&grid, 'S');
    let end_char = 'E';

    let mut visited: Vec<Point> = Vec::new();
    let mut to_visit = vec![start.clone()];

    loop {
        tot += 1;

        let mut next_round = vec![];

        while to_visit.len() > 0 {
            let point = to_visit.pop().unwrap();
            if contains_point(&visited, &point) {
                continue;
            }
            let c_val = lookup.get(&grid[point.y][point.x]).unwrap();
            visited.push(point.clone());

            // up
            if point.y > 0 {
                let new_c = grid[point.y - 1][point.x];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val <= &(*c_val + 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y - 1, point.x);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // down
            if point.y + 1 < grid.len() {
                let new_c = grid[point.y + 1][point.x];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val <= &(*c_val + 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y + 1, point.x);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // right
            if point.x + 1 < grid[point.y].len() {
                let new_c = grid[point.y][point.x + 1];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val <= &(*c_val + 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y, point.x + 1);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // left
            if point.x > 0 {
                let new_c = grid[point.y][point.x - 1];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val <= &(*c_val + 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y, point.x - 1);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }
        }
        // println!("to_visit len: {}",to_visit.len());
        // println!("visited len: {}", visited.len());
        to_visit = next_round;
        if to_visit.len() == 0 {
            panic!("something is wrong!");
        }
    }

    tot
}

fn part2(grid: &Grid) -> i64 {
    let mut tot = 0;
    let lookup = get_lookup();
    let start = get_starting_point(&grid, 'E');
    let end_char = 'a';

    let mut visited: Vec<Point> = Vec::new();
    let mut to_visit = vec![start.clone()];

    loop {
        tot += 1;

        let mut next_round = vec![];

        while to_visit.len() > 0 {
            let point = to_visit.pop().unwrap();
            if contains_point(&visited, &point) {
                continue;
            }
            let c_val = lookup.get(&grid[point.y][point.x]).unwrap();
            visited.push(point.clone());

            // up
            if point.y > 0 {
                let new_c = grid[point.y - 1][point.x];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val >= &(*c_val - 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y - 1, point.x);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // down
            if point.y + 1 < grid.len() {
                let new_c = grid[point.y + 1][point.x];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val >= &(*c_val - 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y + 1, point.x);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // right
            if point.x + 1 < grid[point.y].len() {
                let new_c = grid[point.y][point.x + 1];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val >= &(*c_val - 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y, point.x + 1);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }

            // left
            if point.x > 0 {
                let new_c = grid[point.y][point.x - 1];
                let new_c_val = lookup.get(&new_c).unwrap();
                if new_c_val >= &(*c_val - 1) {
                    if new_c == end_char {
                        return tot;
                    }
                    let new_p = Point::new(point.y, point.x - 1);
                    if !contains_point(&visited, &new_p) {
                        next_round.push(new_p);
                    }
                }
            }
        }
        to_visit = next_round;
        if to_visit.len() == 0 {
            panic!("something is wrong!");
        }
    }
}

fn main() {
    let mut grid = Grid::new();

    read_file("puzzle.txt", parse_items, &mut grid);
    // pretty_print(&grid);

    let p1 = part1(&grid);
    println!("Part 1: {}", p1);

    let p2 = part2(&grid);
    println!("Part 2: {}", p2);
}
