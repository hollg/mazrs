mod generate;
mod grid;

// use generate::binary_tree::binary_tree;
use generate::sidewinder::sidewinder;
use grid::Grid;

fn main() {
    let mut grid = Grid::new(10, 10);

    sidewinder(&mut grid);
    print!("{}", grid.format());
}
