extern crate file_reader;
use file_reader::read_file;

#[derive(Debug)]
struct Round(char, char);

type Game = Vec<Round>;

fn parse_items(s: &str, game: &mut Game) {
    let line = s.parse::<String>().unwrap();

    let round: Vec<char> = line.split(" ").map(|c| c.chars().nth(0).unwrap()).collect();

    game.push(Round(round[0], round[1]));
}

/*
              them  you
    Rock:      A      X
    Paper:     B      Y
    Scissors:  C      Z

    win: 6
    draw: 3
*/

fn part1(game: &Game) -> i64 {
    let mut tot = 0;

    for g in game {
        match g {
            Round('A', 'X') => tot += 4,
            Round('A', 'Y') => tot += 8,
            Round('A', 'Z') => tot += 3,
            Round('B', 'X') => tot += 1,
            Round('B', 'Y') => tot += 5,
            Round('B', 'Z') => tot += 9,
            Round('C', 'X') => tot += 7,
            Round('C', 'Y') => tot += 2,
            Round('C', 'Z') => tot += 6,
            _ => panic!("Bad input.")
        }
    }

    tot
}

/*
    X: lose 0
    Y: draw 3
    Z: win 6

    Rock:      A     1
    Paper:     B     2
    Scissors:  C     3
*/
fn part2(game: &Game) -> i64 {
    let mut tot = 0;
    for g in game {
        match g {
            Round('A', 'X') => tot += 3,
            Round('A', 'Y') => tot += 4,
            Round('A', 'Z') => tot += 8,
            Round('B', 'X') => tot += 1,
            Round('B', 'Y') => tot += 5,
            Round('B', 'Z') => tot += 9,
            Round('C', 'X') => tot += 2,
            Round('C', 'Y') => tot += 6,
            Round('C', 'Z') => tot += 7,
            _ => panic!("Bad input.")
        }
    }

    tot
}

fn main() {
    let mut game = Game::new();

    read_file("puzzle.txt", parse_items, &mut game);

    let p1 = part1(&game);
    let p2 = part2(&game);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
