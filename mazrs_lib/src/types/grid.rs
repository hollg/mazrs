use super::cell::Cell;
use rand::{thread_rng, Rng};
use std::iter;

use std::ops::{Index, IndexMut};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use svg::Node;
pub struct Grid {
    pub height: usize,
    pub width: usize,
    pub cells: Box<[Cell]>,
    // pub links: HashMap<(usize, usize), BTreeSet<(usize, usize)>>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Grid {
        let grid = Grid {
            height,
            width,
            cells: vec![Cell::new(); width * height].into_boxed_slice(),
        };

        return grid;
    }

    // pub fn each_cell<F>(&mut self, mut f: F)
    // where
    //     F: FnMut(&mut Grid, &Cell),
    // {
    //     for cell in self.cells.iter() {
    //         f(self, cell);
    //     }
    // }

    pub fn format(&mut self) -> String {
        let mut ascii = String::new();
        ascii += "+";
        ascii += &iter::repeat("---+").take(self.width).collect::<String>()[..];
        ascii += "\n";
        for y in 0..self.height {
            let row_start_index = y * self.width;
            let row_end_index = row_start_index + self.width;
            let row = &self.cells[row_start_index..row_end_index];

            let mut top = String::from("|");
            let mut bottom = String::from("+");

            for cell in row {
                top += &cell.to_string()[..];

                match cell.is_linked_east {
                    true => top += " ",
                    false => top += "|",
                }
                match cell.is_linked_south {
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

    // pub fn is_linked_indices(&self, i: usize, j: usize) -> bool {
    //     let cell1 = self[i];

    //     match self.links.get(&(x1, y1)) {
    //         Some(set) => match set.get(&(x2, y2)) {
    //             Some(_) => true,
    //             _ => false,
    //         },
    //         None => false,
    //     }
    // }

    pub fn link_cell_north(&mut self, index: usize) {
        let width = self.width;
        self.cells[index].is_linked_north = true;
        self.cells[index - width].is_linked_south = true;
    }

    pub fn link_cell_south(&mut self, index: usize) {
        let width = self.width;
        self[index].is_linked_south = true;
        self[index + width].is_linked_north = true;
    }

    pub fn link_cell_west(&mut self, index: usize) {
        self[index].is_linked_west = true;
        self[index - 1].is_linked_east = true;
    }

    pub fn link_cell_east(&mut self, index: usize) {
        self[index].is_linked_east = true;
        self[index + 1].is_linked_west = true;
    }

    //     match self.links.contains_key(&(cell_b.x, cell_b.y)) {
    //         true => {
    //             self.links
    //                 .get_mut(&(cell_b.x, cell_b.y))
    //                 .unwrap()
    //                 .insert((cell_a.x, cell_a.y));
    //         }
    //         false => {
    //             let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
    //             set.insert((cell_a.x, cell_a.y));
    //             self.links.insert((cell_b.x, cell_b.y), set);
    //         }
    //     }
    // }

    // pub fn links(&mut self, cell: Cell) -> Option<&BTreeSet<(usize, usize)>> {
    //     match self.links.contains_key(&(cell.x, cell.y)) {
    //         true => Some(self.links.get(&(cell.x, cell.y)).unwrap()),
    //         false => None,
    //     }
    // }

    pub fn is_north_boundary(&mut self, index: usize) -> bool {
        index < self.width
    }

    pub fn is_south_boundary(&mut self, index: usize) -> bool {
        index >= self.width * self.height - self.width
    }

    pub fn is_east_boundary(&mut self, index: usize) -> bool {
        (index + 1) % (self.width) == 0
    }
    pub fn is_west_boundary(&mut self, index: usize) -> bool {
        index == 0 || index % self.width != 0
    }

    pub fn link_north(&mut self, index: usize) {
        let mut cell_a = self[index];
        cell_a.link_north();
    }

    pub fn neighbours(&mut self, index: usize) -> Vec<usize> {
        println!("index is: {}", index);
        let mut neighbours = Vec::new();

        // west
        if !self.is_west_boundary(index) {
            neighbours.push(index - 1)
        };

        // east
        if !self.is_east_boundary(index) {
            neighbours.push(index + 1);
        }

        // north
        if !self.is_north_boundary(index) {
            neighbours.push(index - self.width);
        }

        // south
        if !self.is_south_boundary(index) {
            neighbours.push(index + self.width);
        };

        println!("neighbours: {:?}", neighbours);
        return neighbours;
    }

    pub fn random_cell(&self) -> Cell {
        let mut rng = thread_rng();
        let i = rng.gen_range(0, self.width);

        self.cells[i]
    }

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

    pub fn size(&mut self) -> usize {
        self.width * self.height
    }

    pub fn to_svg(&self) -> svg::node::element::SVG {
        let cell_size = 10;
        let img_width = cell_size * self.width;
        let img_height = cell_size * self.height;

        let background_color = "white";
        let wall_color = "black";

        let mut document =
            Document::new().set("viewBox", (-5, -5, img_width + 10, img_height + 10));

        let boundary_wall_data = Data::new()
            .move_to((0, 0))
            .line_to((img_width, 0))
            .line_to((img_width, img_height - cell_size))
            .move_to((img_width, img_height))
            .line_to((0, img_height))
            .line_to((0, cell_size));
        let boundary_wall_path = Path::new()
            .set("fill", background_color)
            .set("stroke", wall_color)
            .set("stroke-width", 1)
            .set("stroke-linejoin", "round")
            .set("stroke-linecap", "round")
            .set("d", boundary_wall_data);
        document.append(boundary_wall_path);

        for y in 0..self.height {
            let row_start_index = y * self.width;
            let row_end_index = row_start_index + self.width + 1;
            let row = &self.cells[row_start_index..row_end_index];

            for (i, cell) in row.iter().enumerate() {
                // let cell = self[x][y];

                // let is_linked_east = self.is_linked_indices(cell.x, cell.y, cell.x + 1, cell.y);
                // let is_linked_south = self.is_linked_indices(cell.x, cell.y, cell.x, cell.y + 1);

                let y = i / self.width;
                let x = i - y * self.width;

                let x1 = x * cell_size;
                let y1 = y * cell_size;
                let x2 = (x + 1) * cell_size;
                let y2 = (y + 1) * cell_size;

                let mut cell_data = Data::new();

                // east wall
                if !cell.is_linked_east && x != self.width - 1 {
                    cell_data = cell_data.move_to((x2, y1)).line_to((x2, y2));
                }

                // south wall
                if !cell.is_linked_south {
                    cell_data = cell_data.move_to((x1, y2)).line_to((x2, y2));
                }

                cell_data = cell_data.close();
                let cell_data_path = Path::new()
                    .set("fill", background_color)
                    .set("stroke", wall_color)
                    .set("stroke-width", 1)
                    .set("stroke-linejoin", "round")
                    .set("stroke-linecap", "round")
                    .set("d", cell_data);
                document.append(cell_data_path);
            }
        }
        document
    }
}

impl Index<usize> for Grid {
    type Output = Cell;

    fn index(&self, index: usize) -> &Cell {
        &self.cells[index]
    }
}
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Cell {
        &mut self.cells[index]
    }
}
