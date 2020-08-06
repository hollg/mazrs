mod generate;
mod grid;

use generate::binary_tree::binary_tree;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(10, 10);

    binary_tree(&mut grid);
    print!("{}", grid.format());
}
