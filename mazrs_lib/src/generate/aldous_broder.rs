// extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::types::grid::Grid;

pub fn generate(grid: &mut Grid) {
    let mut cell = grid.random_cell();
    let mut num_unvisited = grid.size() - 1;
    let mut rng = thread_rng();
    while num_unvisited > 0 {
        let neighbours = grid.neighbours(cell);
        let neighbour = neighbours.choose(&mut rng).unwrap();

        if !grid.links.contains_key(&(neighbour.x, neighbour.y)) {
            grid.link(&cell, &neighbour);
            num_unvisited -= 1;
        }

        cell = *neighbour;
    }
}
