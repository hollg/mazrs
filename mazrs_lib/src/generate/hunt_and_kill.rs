use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::types::grid::*;

// 1. Choose a starting location.
// 2. Perform a random walk, carving passages to unvisited neighbors, until the current cell has no unvisited neighbors.
// 3. Go to the first cell that is both unvisited and adjacent to a visited cell. 
// 4. If found, carve a passage between the two and let the formerly unvisited cell be the new starting location.
// 5. Repeat steps 2 - 4 until there are no unvisited cells.

pub fn generate(grid: &mut Grid) {
    let rng = &mut thread_rng();
    let mut visited_cells = vec![false; grid.size()];
    let neighbours = &mut Vec::with_capacity(4);

    let mut curr_index = (0..grid.size()).choose(rng).unwrap();

    while visited_cells.iter().any(|visited| !visited) {
        visited_cells[curr_index] = true;
        grid.neighbours(curr_index, neighbours);
        neighbours.retain(|neighbour| !visited_cells[*neighbour]);

        if neighbours.is_empty() {
            // if there are no unvisited neighbours, find the first unvisited
            // cell with 1+ visited neighbours, link it to one and visit the
            // neighbour
            for i in 0..grid.size() {
                if visited_cells[i] {
                    continue;
                }

                neighbours.clear();
                grid.neighbours(i, neighbours);
                neighbours.retain(|neighbour| visited_cells[*neighbour]);

                if neighbours.is_empty() {
                    continue;
                }

                grid.link_cells(i, *neighbours.choose(rng).unwrap());
                curr_index = i;
                visited_cells[i] = true;
            }
        } else {
            // if current cell has unvisited neighbours, link to and visit one
            let random_unvisited_neighbour = *neighbours.choose(rng).unwrap();
            grid.link_cells(curr_index, random_unvisited_neighbour);
            curr_index = random_unvisited_neighbour;
            neighbours.clear();
        }
    }
}
