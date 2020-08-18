#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub is_linked_north: bool,
    pub is_linked_south: bool,
    pub is_linked_east: bool,
    pub is_linked_west: bool,
}

impl Cell {
    pub fn new() -> Cell {
        return Cell {
            is_linked_north: false,
            is_linked_south: false,
            is_linked_east: false,
            is_linked_west: false,
        };
    }

    pub fn to_string(&self) -> String {
        return "   ".to_string();
    }

    pub fn link_north(&mut self) {
        self.is_linked_north = true
    }
    pub fn link_south(&mut self) {
        self.is_linked_south = true
    }
    pub fn link_east(&mut self) {
        self.is_linked_east = true
    }
    pub fn link_west(&mut self) {
        self.is_linked_west = true
    }
}
