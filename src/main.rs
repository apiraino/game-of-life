extern crate rand;

use std::{thread, time};
use rand::Rng;

const MAX_X: usize = 40;
const MAX_Y: usize = 20;
const MAX_TICKS: usize = 60;
const TICK_DELAY: u64 = 200;

#[derive(Debug)]
pub enum ArrayOutOfBoundsException {
    WtfError,
}
pub type ArrayResult = Result<usize, &'static str>;

fn init_grid(grid: &mut [[usize; MAX_X]; MAX_Y], zeroed: bool) {
    // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Examples_of_patterns

    if zeroed {
        for coord_y in 0..MAX_Y {
            println!();
            for coord_x in 0..MAX_X {
                grid[coord_y][coord_x] = 0;
            }
        }
    } else {
        // grid[10][2] = 1;
        // grid[11][2] = 1;
        // grid[12][2] = 1;

        grid[10][2] = 1;
        grid[10][3] = 0;
        grid[10][4] = 0;
        grid[11][2] = 0;
        grid[11][3] = 1;
        grid[11][4] = 1;
        grid[12][2] = 1;
        grid[12][3] = 1;
        grid[12][4] = 0;
    }
}

fn get_cell_status(cell: usize, alive_neighbours: usize) -> usize {
    let mut cell_val = 0;
    if cell == 1 {
        if (2 == alive_neighbours) || (3 == alive_neighbours) {
            cell_val = 1;
        }
    } else {
        if alive_neighbours == 3 {
            cell_val = 1;
        }
    }
    cell_val

    // verbose implementation
    // if cell == 1 {
    //     match alive_neighbours {
    //         // cell dies by starving
    //         d if d < 2 => grid[coord_y][coord_x] = 0,

    //         // cell survives
    //         d if (d == 2 || d == 3) => cell_val = 1,

    //         // cell dies by overpopulation
    //         d if d > 3 => grid[coord_y][coord_x] = 0,
    //         _ => panic!("impossible error")
    //     };
    // } else {
    //     // dead cell becomes alive
    //     if alive_neighbours == 3 {
    //         cell_val = 1;
    //     }
    // }
}
fn update_grid(grid: [[usize; MAX_X]; MAX_Y]) -> [[usize; MAX_X]; MAX_Y] {
    let mut new_grid = grid.clone();
    init_grid(&mut new_grid, true);
    for coord_y in 0..MAX_Y {
        for coord_x in 0..MAX_X {
            // FIXME: arrayoutofbounds exception
            if coord_x == 0 || coord_x == MAX_X - 1 {
                continue;
            }
            if coord_y == 0 || coord_y == MAX_Y - 1 {
                continue;
            }

            // checking my neighbours
            let tl = grid[coord_y - 1][coord_x - 1];
            let tc = grid[coord_y - 1][coord_x];
            let tr = grid[coord_y - 1][coord_x + 1];

            let cl = grid[coord_y][coord_x - 1];
            let cell = grid[coord_y][coord_x];
            let cr = grid[coord_y][coord_x + 1];

            let bl = grid[coord_y + 1][coord_x - 1];
            let bc = grid[coord_y + 1][coord_x];
            let br = grid[coord_y + 1][coord_x + 1];

            let alive_neighbours = tl + tc + tr + cl + cr + bl + bc + br;
            new_grid[coord_y][coord_x] = get_cell_status(cell, alive_neighbours);
        }
    }
    new_grid
}

fn print_grid(grid: &mut [[usize; MAX_X]; MAX_Y]) {
    for coord_y in 0..MAX_Y {
        println!();
        for coord_x in 0..MAX_X {
            print!("{}", grid[coord_y][coord_x]);
        }
    }
}

fn main() {
    // also: https://stackoverflow.com/a/36376568
    // notice: grid[y][x]
    let mut grid = [[0; MAX_X]; MAX_Y];
    let mut num_tick = 0;

    // #[macro_use(c)]
    // extern crate cute;
    // let vector = c![x, for x in 1..10, if x % 2 == 0];

    init_grid(&mut grid, false);
    while num_tick <= MAX_TICKS {
        let rand_str = rand::thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect::<String>();
        // https://stackoverflow.com/a/34837038
        print!("{}[2J", 27 as char);
        println!("\n\nTick: {} ---- [{}]", num_tick, rand_str);
        print_grid(&mut grid);
        let new_grid = update_grid(grid);

        thread::sleep(time::Duration::from_millis(TICK_DELAY));
        num_tick += 1;

        grid = new_grid;
    }

    println!();
}
