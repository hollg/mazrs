use std::collections::BTreeSet;
use std::collections::HashMap;
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Cell>>,
    links: HashMap<(usize, usize), BTreeSet<(usize, usize)>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        let mut grid = Grid {
            height,
            width,
            cells: Vec::with_capacity(width),
            links: HashMap::new(),
        };

        for i in 0..width {
            let mut col: Vec<Cell> = Vec::with_capacity(height);

            for j in 0..height {
                col.push(Cell::new(i, j))
            }

            grid.cells.push(col)
        }

        return grid;
    }

    pub fn is_linked(&mut self, cell_a: Cell, cell_b: Cell) -> bool {
        match self.links.get(&(cell_a.x, cell_a.y)) {
            Some(set) => match set.get(&(cell_b.x, cell_b.y)) {
                Some(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn link(&mut self, cell_a: &Cell, cell_b: &Cell) {
        match self.links.contains_key(&(cell_a.x, cell_a.y)) {
            true => {
                self.links
                    .get_mut(&(cell_a.x, cell_a.y))
                    .unwrap()
                    .insert((cell_b.x, cell_b.y));
            }
            false => {
                let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
                set.insert((cell_b.x, cell_b.y));
                self.links.insert((cell_a.x, cell_a.y), set);
            }
        }

        match self.links.contains_key(&(cell_b.x, cell_b.y)) {
            true => {
                self.links
                    .get_mut(&(cell_b.x, cell_b.y))
                    .unwrap()
                    .insert((cell_a.x, cell_a.y));
            }
            false => {
                let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
                set.insert((cell_a.x, cell_a.y));
                self.links.insert((cell_b.x, cell_b.y), set);
            }
        }
    }

    pub fn links(&mut self, cell: Cell) -> Option<&BTreeSet<(usize, usize)>> {
        match self.links.contains_key(&(cell.x, cell.y)) {
            true => Some(self.links.get(&(cell.x, cell.y)).unwrap()),
            false => None,
        }
    }

    pub fn unlink(&mut self, cell_a: &Cell, cell_b: &Cell) {
        match self.links.contains_key(&(cell_a.x, cell_a.y)) {
            true => {
                let set = self.links.get_mut(&(cell_a.x, cell_a.y)).unwrap();
                set.remove(&(cell_b.x, cell_b.y));

                if set.is_empty() {
                    self.links.remove(&(cell_a.x, cell_a.y));
                }
            }
            false => {}
        }

        match self.links.contains_key(&(cell_b.x, cell_b.y)) {
            true => {
                let set = self.links.get_mut(&(cell_b.x, cell_b.y)).unwrap();
                set.remove(&(cell_a.x, cell_a.y));

                if set.is_empty() {
                    self.links.remove(&(cell_b.x, cell_b.y));
                }
            }
            false => {}
        }
    }
}

struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        return Cell { x, y };
    }
}

fn main() {
    let grid = Grid::new(4, 4);
}
