extern crate rand;

use crate::types::grid::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn generate(grid: &mut Grid) {
    let mut unvisited_cells: Vec<(usize, usize)> = vec![];
    let mut rng = thread_rng();
    for x in 0..grid.width {
        for y in 0..grid.height {
            unvisited_cells.push((x, y));
        }
    }

    let first_target = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
    println!("first target: {:?}", first_target);
    unvisited_cells.retain(|&coords| coords != first_target);

    while unvisited_cells.len() > 0 {
        let mut current_cell = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
        println!("current_cell: {:?}", current_cell);
        let mut path: Vec<(usize, usize)> = vec![current_cell];

        while unvisited_cells.contains(&current_cell) {
            let neighbours = grid.neighbours(grid[current_cell.0][current_cell.1]);
            let random_neighbour = neighbours.choose(&mut rand::thread_rng()).unwrap();
            current_cell = (random_neighbour.x, random_neighbour.y);
            println!("current_cell: {:?}", current_cell);
            match path.binary_search(&current_cell) {
                Ok(index) => {
                    path.truncate(index +1);
                }
                _ => {
                    path.push(current_cell);
                }
            }
        }

        for window in path.windows(2) {
            let a = window[0];
            let b = window[1];
            let cell_a = grid[a.0][a.1];
            let cell_b = grid[b.0][b.1];

            grid.link(&cell_a, &cell_b);
            unvisited_cells.retain(|&x| x != a && x != b);
        }

        println!("{}", grid.format());
    }
}
