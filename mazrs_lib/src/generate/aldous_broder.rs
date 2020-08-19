// extern crate rand;

use crate::types::grid::Grid;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

pub fn generate(grid: &mut Grid) {
    let mut rng = thread_rng();
    let mut visited_cells = vec![false; grid.size()];
    let mut curr_index = (0..grid.size()).choose(&mut rng).unwrap();
    visited_cells[0] = true;
    while visited_cells.iter().any(|visited| !visited) {
        println!("{:?}", visited_cells);
        let neighbours = grid.neighbours(curr_index);
        let neighbour_index = neighbours.choose(&mut rng).unwrap();

        if !visited_cells[*neighbour_index] {
            if neighbour_index + grid.width == curr_index {
                println!("linking north");
                grid.link_cell_north(curr_index);
            }

            if neighbour_index >= &grid.width && neighbour_index - grid.width == curr_index {
                println!("linking south");
                grid.link_cell_south(curr_index);
            }
            if neighbour_index + 1 == curr_index {
                println!("linking east");
                grid.link_cell_east(curr_index);
            }
            if neighbour_index >= &1 && neighbour_index - 1 == curr_index {
                println!("linking west");
                grid.link_cell_west(curr_index);
            }
        }
        visited_cells[curr_index] = true;
        curr_index = *neighbour_index;
    }

    // let mut cell = grid.random_cell();
    // let mut num_unvisited = grid.size() - 1;
    // let mut rng = thread_rng();
    // while num_unvisited > 0 {
    //     let neighbours = grid.neighbours(cell);
    //     let neighbour = neighbours.choose(&mut rng).unwrap();

    //     if !grid.links.contains_key(&(neighbour.x, neighbour.y)) {
    //         grid.link(&cell, &neighbour);
    //         num_unvisited -= 1;
    //     }

    //     cell = *neighbour;
    // }
}
