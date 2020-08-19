extern crate rand;

use crate::types::grid::Grid;
use rand::seq::SliceRandom;
use rand::thread_rng;

enum Direction {
    North,
    East,
}

pub fn generate(grid: &mut Grid) {
    for i in 0..grid.cells.len() {
        let mut possible_links: Vec<Direction> = Vec::new();

        if !grid.is_north_boundary(i) {
            possible_links.push(Direction::North)
        }

        if !grid.is_east_boundary(i) {
            possible_links.push(Direction::East)
        }

        if possible_links.len() > 0 {
            let mut rng = thread_rng();
            match possible_links.choose(&mut rng).unwrap() {
                Direction::North => {
                    grid.cells[i].link_north();
                    grid.cells[i - grid.width].link_south();
                }
                Direction::East => {
                    grid.cells[i].link_east();
                    grid.cells[i + 1].link_west();
                }
            }
        }
    }
}
