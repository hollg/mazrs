use crate::types::grid::Grid;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate(grid: &mut Grid) {
    let rng = &mut thread_rng();
    let mut visited_cells = vec![false; grid.size()];
    let mut neighbours: Vec<usize> = Vec::with_capacity(4);
    let mut path: Vec<usize> = vec![(0..grid.size()).choose(rng).unwrap()];

    visited_cells[0] = true;

    while visited_cells.iter().any(|visited| !visited) {
        // when the path hits a visited cell, link it up
        if visited_cells[*path.last().unwrap()] {
            for window in path.windows(2) {
                visited_cells[window[0]] = true;
                visited_cells[window[1]] = true;
                grid.link_cells(window[0], window[1]);
            }
            path.clear();
            path.push((0..grid.size()).choose(rng).unwrap());
        }
        // else pick a random neighbour to add to the path
        else {
            neighbours.clear();
            grid.neighbours(*path.last().unwrap(), &mut neighbours);

            let target = *neighbours.choose(rng).unwrap();

            // if we hit the current path, clear the loop
            if let Some(i) = path.iter().position(|i| *i == target) {
                path.truncate(i + 1);
            } else {
                path.push(target);
            }
        }
    }
}
