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

    pub fn link_cells(&mut self, index_a: usize, index_b: usize) {
        if index_b + self.width == index_a {
            println!("linking north");
            self.link_cell_north(index_a);
        }

        if index_b >= self.width && index_b - self.width == index_a {
            println!("linking south");
            self.link_cell_south(index_a);
        }
        if index_b + 1 == index_a {
            println!("linking west");
            self.link_cell_west(index_a);
            
        }
        if index_b >= 1 && index_b - 1 == index_a {
            println!("linking east");
            self.link_cell_east(index_a);
        }
    }

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

    pub fn is_north_boundary(&self, index: usize) -> bool {
        index < self.width
    }

    pub fn is_south_boundary(&self, index: usize) -> bool {
        index >= self.size() - self.width
    }

    pub fn is_east_boundary(&self, index: usize) -> bool {
        let is_east = (index + 1) % (self.width) == 0;
        println!("is_east {} {}", index, is_east);
        is_east
    }
    pub fn is_west_boundary(&self, index: usize) -> bool {
        index == 0 || index % self.width == 0
    }

    pub fn neighbours(&self, index: usize, buf: &mut Vec<usize>) {
        if !self.is_north_boundary(index) {
            buf.push(index - self.width);
        }
        if !self.is_south_boundary(index) {
            buf.push(index + self.width);
        }
        if !self.is_east_boundary(index) {
            buf.push(index + 1);
        }
        if !self.is_west_boundary(index) {
            buf.push(index - 1);
        }
    }

    pub fn random_cell(&self) -> Cell {
        let mut rng = thread_rng();
        let i = rng.gen_range(0, self.width);

        self.cells[i]
    }

    pub fn size(&self) -> usize {
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
