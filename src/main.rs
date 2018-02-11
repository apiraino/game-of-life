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
pub type ArrayElementResult = Result<usize, &'static str>;

fn init_grid(grid: &mut [[usize; MAX_X]; MAX_Y]) {
    // https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Examples_of_patterns

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
}

fn get_pos_value(
    // Check the requested coordinate againts overflows
    // returns 0 if an overflow is detected
    // else returns the grid coordinate value
    grid: [[usize; MAX_X]; MAX_Y],
    y: usize,
    action_y: &str,
    x: usize,
    action_x: &str,
) -> usize {
    let checked_y;
    let checked_x;

    if action_y == "dec" {
        // returns 0 if overflows
        if y.saturating_sub(1) != 0 {
            checked_y = y - 1;
        } else {
            return 0;
        }
    } else if action_y == "inc" {
        if y + 1 < MAX_Y {
            checked_y = y + 1;
        } else {
            return 0;
        }
    } else {
        checked_y = y;
    }

    if action_x == "dec" {
        if x.saturating_sub(1) != 0 {
            checked_x = x - 1;
        } else {
            return 0;
        }
    } else if action_x == "inc" {
        if x + 1 < MAX_X {
            checked_x = x + 1;
        } else {
            return 0;
        }
    } else {
        checked_x = x;
    }

    // coordinates are valid and inbound
    grid[checked_y][checked_x]
}

fn update_grid(grid: [[usize; MAX_X]; MAX_Y]) -> [[usize; MAX_X]; MAX_Y] {
    let mut new_grid = [[0; MAX_X]; MAX_Y];
    for coord_y in 0..MAX_Y {
        for coord_x in 0..MAX_X {
            // checking my neighbours
            let tl = get_pos_value(grid, coord_y, "dec", coord_x, "dec");
            let tc = get_pos_value(grid, coord_y, "dec", coord_x, "");
            let tr = get_pos_value(grid, coord_y, "dec", coord_x, "inc");

            let cl = get_pos_value(grid, coord_y, "", coord_x, "dec");
            let cell = get_pos_value(grid, coord_y, "", coord_x, "");
            let cr = get_pos_value(grid, coord_y, "", coord_x, "inc");

            let bl = get_pos_value(grid, coord_y, "inc", coord_x, "dec");
            let bc = get_pos_value(grid, coord_y, "inc", coord_x, "");
            let br = get_pos_value(grid, coord_y, "inc", coord_x, "inc");

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
    // type is inferred, i.e.:
    // let grid: [[usize; 12]; 12] = [[7;12]; 12];
    let mut grid = [[0; MAX_X]; MAX_Y];
    let mut num_tick = 0;

    // unrelated: check this
    // #[macro_use(c)]
    // extern crate cute;
    // let vector = c![x, for x in 1..10, if x % 2 == 0];

    init_grid(&mut grid);
    while num_tick <= MAX_TICKS {
        let rand_str = rand::thread_rng()
            .gen_ascii_chars()
            .take(10)
            .collect::<String>();
        // https://stackoverflow.com/a/34837038
        // clear screen
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
