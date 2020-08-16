use mazrs_lib::{generate::aldous_broder, types::grid::*};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn generate() -> String {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut grid = Grid::new(20, 20);
    aldous_broder::generate(&mut grid);

    let svg = grid.to_svg();
    svg.to_string()
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
