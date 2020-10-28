use mazrs_lib::{generate::hunt_and_kill, types::grid::*};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn generate(height: usize, width: usize) -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut grid = Grid::new(height, width);
    hunt_and_kill::generate(&mut grid);

    let svg = grid.to_svg();
    svg.to_string()
}
