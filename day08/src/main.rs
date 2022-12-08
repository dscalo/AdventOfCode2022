extern crate file_reader;
use file_reader::read_file;


type Grid = Vec<Vec<u32>>;

fn parse_items(s: &str, items: &mut Grid) {
    let line = s.parse::<String>().unwrap();
    
    let row: Vec<u32> = line.chars().map(|n| n.to_string().parse::<u32>().unwrap()).collect();

    items.push(row);

}

fn pretty_print(grid: &Grid) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}",grid[y][x])
        }
        print!("\n")
    }
    print!("\n")
}

/*


                30373
                25512
                65332
                33549
                35390


*/

fn is_visible(grid: &Grid, y: usize, x: usize) -> bool {
    let mut is_vis = true;
    
    let num = grid[y][x];

    // up
    let mut up  = y - 1;
    loop {
        
        if  grid[up][x] >= num {
            is_vis = false;
            break;
        }
        if up == 0 {
            break;
        }
        up -= 1;
    }

    if is_vis {
        return is_vis;
    }

    is_vis = true;

    // right
    let mut right = x + 1;
    while right < grid[y].len() {
        if grid[y][right] >= num {
            is_vis = false;
            break;
        }
        right += 1;
    }

    if is_vis {
        return is_vis;
    }

    is_vis = true;
    // down
    let mut down = y + 1;
    while down < grid.len() {
        if  grid[down][x] >= num {
            is_vis = false;
            break;
        }
        down += 1;
    }

    if is_vis {
        return is_vis;
    }


    is_vis = true;
    // left
    let mut left = x -1;
    loop {
        if grid[y][left] >= num {
            is_vis = false;
            break;
        }

        if left == 0 {
            break;
        }
        left -= 1;
    }


    is_vis
}

fn part1(grid: &Grid) -> i64 {
    let mut tot = 0;
    
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() -1 {
            if is_visible(&grid, y, x) {
                tot += 1;
            }
        }
    }

    let y_len = grid.len() as i64 * 2;
    let x_len = grid[0].len() as i64 * 2;

    
    
   tot += y_len + (x_len - 4);

    tot
}

fn get_viewing_distance(grid: &Grid, y: usize, x: usize) -> i64 {
       
    let num = grid[y][x];

    // up
    let mut up  = y - 1;
    let mut up_score = 0;
    loop {
        
        up_score += 1;
        if grid[up][x] >= num {           
            break;
        }

        if up == 0 {
            break;
        }
        up -= 1;
    }


    // right
    let mut right = x + 1;
    let mut right_score = 0;
    while right < grid[y].len() {
        right_score += 1;
        if grid[y][right] >= num {
            break;
        }
        right += 1;
    }

      // down
      let mut down = y + 1;
      let mut down_score = 0;
      while down < grid.len() {
        down_score += 1;
          if  grid[down][x] >= num {
              break;
          }
          down += 1;
      }
    
    // left
    let mut left = x -1;
    let mut left_score = 0;
    loop {
        left_score += 1;
        if grid[y][left] >= num {
            break;
        }

        if left == 0 {
            break;
        }
        left -= 1;
    }



    up_score * right_score * down_score * left_score
}

fn part2(grid: &Grid) -> i64 {
    let mut tot = 0;
    
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() -1 {
            let score = get_viewing_distance(&grid, y, x);
            if score > tot {
                tot = score
            }
        }
    }   

    tot
}




fn main() {
    let mut grid = Grid::new();
    read_file("puzzle.txt", parse_items, &mut grid);

    //pretty_print(&grid);

    let p1 = part1(&grid);
    let p2 = part2(&grid);


    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
