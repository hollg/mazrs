extern crate rand;

use rand::{thread_rng, Rng};

use crate::types::grid::Grid;

pub fn generate(grid: &mut Grid) {
    let mut cell = grid.random_cell();
    let mut num_unvisited = grid.size() - 1;
    let mut rng = thread_rng();
    while num_unvisited > 0 {
        let neighbours = grid.neighbours(cell);
        let neighbour = neighbours[rng.gen_range(0, neighbours.len())];

        if !grid.links.contains_key(&(neighbour.x, neighbour.y)) {
            grid.link(&cell, &neighbour);
            num_unvisited -= 1;
        }

        cell = neighbour;
    }
}
