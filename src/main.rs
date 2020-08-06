mod args;
mod generate;
mod grid;

use args::{Algorithm, Output};
use generate::binary_tree::binary_tree;
use generate::sidewinder::sidewinder;
use grid::Grid;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let algorithm = matches.value_of("algorithm").unwrap().parse();
    let output = matches.value_of("output").unwrap().parse();

    let mut grid = Grid::new(10, 10);

    match algorithm {
        Ok(Algorithm::Binary) => binary_tree(&mut grid),
        Ok(Algorithm::Sidewinder) => sidewinder(&mut grid),
        Err(_) => panic!("Invalid algorithm argument"),
    }

    match output {
        Ok(Output::Ascii) => {
            print!("{}", grid.format());
        }
        Err(_) => panic!("Invalid output argument"),
    }
}
