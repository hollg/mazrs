extern crate rand;

use rand::{thread_rng, Rng};

use crate::types::grid::Grid;

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
        println!("unvisited cells: {}", unvisited_cells.len());
        let mut cell = unvisited_cells[rng.gen_range(0, unvisited_cells.len())];
        let mut path: Vec<(usize, usize)> = vec![cell];

        while unvisited_cells.contains(&cell) {
            let neighbours = grid.neighbours(grid[cell.0][cell.1]);
            let random_neighbour = neighbours[rng.gen_range(0, neighbours.len())];
            cell = (random_neighbour.x, random_neighbour.y);

            match path.binary_search(&cell) {
                Ok(index) => {
                    path = path[0..index].to_vec();
                }
                _ => {
                    path.push(cell);
                }
            }
        }

        let upper = path.len() - 2;
        for i in 0..upper {
            let a = path[i];
            let b = path[i + 1];
            let cell_a = grid[a.0][a.1];
            let cell_b = grid[b.0][b.1];
            grid.link(&cell_a, &cell_b);
            unvisited_cells.retain(|&x| x != a);
        }
    }
}
