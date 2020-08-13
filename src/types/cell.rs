#[derive(Clone, Copy, PartialEq)]
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
