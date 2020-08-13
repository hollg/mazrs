extern crate rand;

use crate::types::grid::*;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

pub fn generate(grid: &mut Grid) {
    // init
    let mut unvisited_cells: Vec<(usize, usize)> = vec![];
    let mut rng = thread_rng();
    for x in 0..grid.width {
        for y in 0..grid.height {
            unvisited_cells.push((x, y));
        }
    }

    let first_to_visit = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
    unvisited_cells.retain(|&coords| coords != first_to_visit);

    // build path
    while unvisited_cells.len() > 0 {
        // let mut current_cell = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
        let mut path: Vec<(usize, usize)> =
            vec![unvisited_cells[rng.gen_range(0, unvisited_cells.len())]];

        while unvisited_cells.contains(path.last().unwrap()) {
            let neighbours = grid.neighbours(grid[path.last().unwrap().0][path.last().unwrap().1]);
            let random_neighbour = neighbours.choose(&mut rng).unwrap();
            let target = (random_neighbour.x, random_neighbour.y);

            match path.binary_search(&target) {
                Ok(index) => {
                    path.truncate(index + 1);
                }
                _ => {
                    path.push(target);
                }
            }
        }

        // carve
        for window in path.windows(2) {
            let cell_a = grid[window[0].0][window[0].1];
            let cell_b = grid[window[1].0][window[1].1];
            grid.link(&cell_a, &cell_b);
            unvisited_cells.retain(|&x| x != window[0] && x != window[1]);
        }
    }
}
