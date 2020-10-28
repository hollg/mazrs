use mazrs_lib::{generate::binary_tree, types::grid::*};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn generate() -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut grid = Grid::new(20, 20);
    binary_tree::generate(&mut grid);

    let svg = grid.to_svg();
    svg.to_string()
}
