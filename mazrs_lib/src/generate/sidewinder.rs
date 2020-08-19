extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::types::grid::*;

pub fn generate(grid: &mut Grid) {
    let mut rng = thread_rng();
    let mut run: Vec<usize> = Vec::new();
    for i in 0..grid.size() - 1 {
        run.push(i);

        let mut should_clear = false;

        if grid.is_east_boundary(i) || (!grid.is_north_boundary(i) && rng.gen_range(0, 2) == 0) {
            let member = *run.choose(&mut rng).unwrap();
            if !grid.is_north_boundary(i) {
                grid.link_cell_north(member);
            }
            should_clear = true;
        } else {
            grid.link_cell_east(i);
        }

        if should_clear {
            run.clear();
        }
    }
}
