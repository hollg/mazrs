extern crate rand;

use rand::{thread_rng, Rng};

mod grid;
use grid::{Cell, Grid};

fn binary_tree(grid: &mut Grid) {
    grid.each_cell(|grid, cell| {
        let mut north_and_east: Vec<Cell> = Vec::new();

        if cell.x < (grid.width - 1) {
            north_and_east.push(grid[cell.x + 1][cell.y].clone());
        }
        if cell.y < (grid.height - 1) {
            north_and_east.push(grid[cell.x][cell.y + 1].clone());
        }

        if north_and_east.len() > 0 {
            let mut rng = thread_rng();
            let i: usize = rng.gen_range(0, north_and_east.len()); 
            grid.link(cell, &north_and_east[i])
        }
    })
}

fn main() {
    let mut grid = Grid::new(10, 10);

    binary_tree(&mut grid);
    print!("{}", grid.format());
}
