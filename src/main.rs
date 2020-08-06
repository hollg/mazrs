mod generate;
mod grid;
#[macro_use]
extern crate clap;
use clap::App;
use generate::binary_tree::binary_tree;
use generate::sidewinder::sidewinder;
use grid::Grid;
use std::str::FromStr;

enum Algorithm {
    Binary,
    Sidewinder,
}

impl FromStr for Algorithm {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "binary" => Ok(Algorithm::Binary),
            "sidewinder" => Ok(Algorithm::Sidewinder),
            _ => Err("no match"),
        }
    }
}

enum Output {
    Ascii,
}

impl FromStr for Output {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Output::Ascii),
            _ => Err("no match"),
        }
    }
}
fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let algorithm = Algorithm::from_str(matches.value_of("algorithm").unwrap());
    let output = Output::from_str(matches.value_of("output").unwrap());

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
