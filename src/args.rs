use std::str::FromStr;

pub enum Algorithm {
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

pub enum Output {
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
