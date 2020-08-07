use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter;
use std::ops::Index;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use svg::Node;
pub struct Grid {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Vec<Cell>>,
    pub links: HashMap<(usize, usize), BTreeSet<(usize, usize)>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        let mut grid = Grid {
            height,
            width,
            cells: Vec::with_capacity(width),
            links: HashMap::new(),
        };

        for x in 0..width {
            let mut col: Vec<Cell> = Vec::with_capacity(height);

            for y in 0..height {
                col.push(Cell::new(x, y))
            }

            grid.cells.push(col)
        }

        return grid;
    }

    pub fn each_cell<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Grid, &Cell),
    {
        for x in 0..self.width {
            for y in 0..self.height {
                f(self, &mut self.cells[x][y].clone());
            }
        }
    }

    pub fn format(&mut self) -> String {
        let mut ascii = String::new();
        ascii += "+";
        ascii += &iter::repeat("---+").take(self.width).collect::<String>()[..];
        ascii += "\n";
        for y in 0..self.height {
            let mut top = String::from("|");
            let mut bottom = String::from("+");
            for x in 0..self.width {
                top += &self.cells[x][y].to_string()[..];

                match self.is_linked_indices(x, y, x + 1, y) {
                    true => top += " ",
                    false => top += "|",
                }
                match self.is_linked_indices(x, y, x, y + 1) {
                    true => bottom += "   +",
                    false => bottom += "---+",
                }
            }

            ascii += &top[..];
            ascii += "\n";

            ascii += &bottom[..];
            ascii += "\n";
        }
        return ascii;
    }

    // pub fn is_linked(&mut self, cell_a: Cell, cell_b: Cell) -> bool {
    //     match self.links.get(&(cell_a.x, cell_a.y)) {
    //         Some(set) => match set.get(&(cell_b.x, cell_b.y)) {
    //             Some(_) => true,
    //             _ => false,
    //         },
    //         _ => false,
    //     }
    // }

    pub fn is_linked_indices(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.links.get(&(x1, y1)) {
            Some(set) => match set.get(&(x2, y2)) {
                Some(_) => true,
                _ => false,
            },
            None => false,
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

    // pub fn links(&mut self, cell: Cell) -> Option<&BTreeSet<(usize, usize)>> {
    //     match self.links.contains_key(&(cell.x, cell.y)) {
    //         true => Some(self.links.get(&(cell.x, cell.y)).unwrap()),
    //         false => None,
    //     }
    // }

    // pub fn neighbours(&mut self, cell: Cell) -> Vec<Cell> {
    //     let mut neighbours = Vec::new();

    //     if cell.x > 0 {
    //         neighbours.push(self.cells[cell.x - 1][cell.y].clone())
    //     };

    //     if cell.x < self.width - 1 {
    //         neighbours.push(self.cells[cell.x + 1][cell.y].clone());
    //     }

    //     if cell.y > 0 {
    //         neighbours.push(self.cells[cell.x][cell.y - 1].clone());
    //     }

    //     if cell.y < self.height - 1 {
    //         neighbours.push(self.cells[cell.x][cell.y + 1].clone());
    //     };

    //     return neighbours;
    // }

    // pub fn random_cell(&self) -> Cell {
    //     let between_x = Range::new(0, self.width);
    //     let between_y = Range::new(0, self.height);
    //     let mut rng = rand::thread_rng();

    //     let x = between_x.ind_sample(&mut rng);
    //     let y = between_y.ind_sample(&mut rng);

    //     self.cells[x][y].clone()
    // }

    // pub fn unlink(&mut self, cell_a: &Cell, cell_b: &Cell) {
    //     match self.links.contains_key(&(cell_a.x, cell_a.y)) {
    //         true => {
    //             let set = self.links.get_mut(&(cell_a.x, cell_a.y)).unwrap();
    //             set.remove(&(cell_b.x, cell_b.y));

    //             if set.is_empty() {
    //                 self.links.remove(&(cell_a.x, cell_a.y));
    //             }
    //         }
    //         false => {}
    //     }

    //     match self.links.contains_key(&(cell_b.x, cell_b.y)) {
    //         true => {
    //             let set = self.links.get_mut(&(cell_b.x, cell_b.y)).unwrap();
    //             set.remove(&(cell_a.x, cell_a.y));

    //             if set.is_empty() {
    //                 self.links.remove(&(cell_b.x, cell_b.y));
    //             }
    //         }
    //         false => {}
    //     }
    // }

    // pub fn size(&mut self) -> usize {
    //     self.width * self.height
    // }

    pub fn to_svg(&mut self) {
        let cell_size = 10;
        let img_width = cell_size * self.width;
        let img_height = cell_size * self.height;

        let background_color = "white";
        let wall_color = "black";

        let mut document = Document::new().set("viewBox", (0, 0, img_width, img_height));

        self.each_cell(|_grid, cell| {
            let x1 = cell.x * cell_size;
            let y1 = cell.y * cell_size;
            // let x2 = cell.x * cell_size;
            let y2 = cell.y * cell_size;

            let data = Data::new()
                .move_to((x1, y1))
                .line_by((x1, y2))
                // .line_by((x2, y2))
                // .line_by((x1, y2))
                .close();

            let path = Path::new()
                .set("fill", background_color)
                .set("stroke", wall_color)
                .set("stroke-width", 1)
                .set("d", data);

            document.append(path);
        });

        svg::save("maze.svg", &document).unwrap();
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Cell>;

    fn index<'a>(&'a self, index: usize) -> &'a Vec<Cell> {
        &self.cells[index]
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        return Cell { x, y };
    }

    pub fn to_string(&self) -> String {
        return "   ".to_string();
    }
}
