mod args;
mod generate;
// mod grid;
mod types;
use args::{Algorithm, Output};
use generate::{aldous_broder, binary_tree, sidewinder, wilson};
// use grid::Grid;
use types::grid::*;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let algorithm = matches.value_of("algorithm").unwrap().parse();
    let output = matches.value_of("output").unwrap().parse();
    let height = matches
        .value_of("height")
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();
    let width = matches
        .value_of("width")
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap();

    let mut grid = Grid::new(height, width);

    match algorithm {
        Ok(Algorithm::Binary) => binary_tree::generate(&mut grid),
        Ok(Algorithm::Sidewinder) => sidewinder::generate(&mut grid),
        Ok(Algorithm::AldousBroder) => aldous_broder::generate(&mut grid),
        Ok(Algorithm::Wilson) => wilson::generate(&mut grid),
        Err(_) => panic!("Invalid algorithm argument"),
    }

    match output {
        Ok(Output::Ascii) => {
            print!("{}", grid.format());
        }
        Ok(Output::Svg) => {
            grid.to_svg();
        }
        Err(_) => panic!("Invalid output argument"),
    }
}
