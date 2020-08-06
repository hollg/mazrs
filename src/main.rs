use rand::Rng;
extern crate rand;

mod grid;
use grid::{Cell, Grid};

fn binary_tree(grid: &mut Grid) {
    grid.each_cell(|grid, cell| {
        let mut cells: Vec<Cell> = Vec::new();

        if cell.x < (grid.width - 1) {
            cells.push(grid[cell.x + 1][cell.y].clone());
        }
        if cell.y < (grid.height - 1) {
            cells.push(grid[cell.x][cell.y + 1].clone());
        }

        if cells.len() > 0 {
            grid.link(cell, rand::thread_rng().choose(&cells).unwrap())
        }
    })
}

fn main() {
    let mut grid = Grid::new(10, 10);

    binary_tree(&mut grid);
    print!("{}", grid.format());
}
