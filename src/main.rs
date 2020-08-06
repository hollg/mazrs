mod generate;
mod grid;
#[macro_use]
extern crate clap;
use clap::App;
use generate::binary_tree::binary_tree;
use generate::sidewinder::sidewinder;
use grid::Grid;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let algorithm = matches.value_of("algorithm").unwrap_or("sidewinder");
    let output = matches.value_of("output").unwrap_or("ascii");

    let mut grid = Grid::new(10, 10);

    match algorithm {
        "binary" => binary_tree(&mut grid),
        "sidewinder" => sidewinder(&mut grid),
        _ => panic!("Invalid algorithm argument"),
    }

    match output {
        "ascii" => {
            print!("{}", grid.format());
        }
        _ => panic!("Invalid output argument"),
    }
}
