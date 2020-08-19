use crate::types::grid::Grid;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate(grid: &mut Grid) {
    let mut rng = thread_rng();
    let mut visited_cells = vec![false; grid.size()];

    let mut curr_index = (0..grid.size()).choose(&mut rng).unwrap();
    let mut neighbours = Vec::with_capacity(4);

    visited_cells[curr_index] = true;
    while visited_cells.iter().any(|visited| !visited) {
        neighbours.clear();
        grid.neighbours(curr_index, &mut neighbours);
        println!("cell: {}", curr_index);
        println!("neighbours: {:?}", neighbours);
        let neighbour_index = *neighbours.choose(&mut rng).unwrap();
        if !visited_cells[neighbour_index] {
            grid.link_cells(curr_index, neighbour_index);
        }
        curr_index = neighbour_index;
        visited_cells[curr_index] = true;
    }
}
