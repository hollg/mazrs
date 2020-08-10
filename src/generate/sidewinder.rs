extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::grid::{Cell, Grid};

pub fn sidewinder(grid: &mut Grid) {
    for y in 0..grid.height {
        let mut run: Vec<Cell> = Vec::new();

        for x in 0..grid.width {
            run.push(grid[x][y]);

            let at_eastern_boundary = x == grid.width - 1;
            let at_northern_boundary = y == grid.height - 1;

            let mut rng = thread_rng();
            let should_close_out =
                at_eastern_boundary || !at_northern_boundary && rng.gen_range(0, 2) == 0;

            let mut should_clear = false;
            if should_close_out {
                let member = run.choose(&mut rng).unwrap();
                if !at_northern_boundary {
                    let north_neighbour = grid[member.x][member.y + 1];
                    grid.link(member, &north_neighbour);
                }
                should_clear = true;
            } else {
                let cell_a = grid[x][y];
                let cell_b = grid[x + 1][y];

                grid.link(&cell_a, &cell_b);
            }
            if should_clear {
                run.clear();
            }
        }
    }
}
