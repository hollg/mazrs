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

    let first_to_visit = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
    unvisited_cells.retain(|&coords| coords != first_to_visit);

    while unvisited_cells.len() > 0 {
        let mut current_cell = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
        let mut path: Vec<(usize, usize)> = vec![current_cell];

        while unvisited_cells.contains(&current_cell) {
            let neighbours = grid.neighbours(grid[current_cell.0][current_cell.1]);
            let random_neighbour = neighbours.choose(&mut rand::thread_rng()).unwrap();
            current_cell = (random_neighbour.x, random_neighbour.y);

            match path.binary_search(&current_cell) {
                Ok(index) => {
                    path = path[0..index + 1].to_vec();
                }
                _ => {
                    path.push(current_cell);
                }
            }
        }

        for i in 0..path.len() - 1 {
            if let Some(j) = path.get(i + 1) {
                let a = path[i];
                let cell_a = grid[a.0][a.1];
                let cell_b = grid[j.0][j.1];
                grid.link(&cell_a, &cell_b);
                unvisited_cells.retain(|&x| x != a);
            }
        }
    }
}
