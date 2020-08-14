use std::str::FromStr;

pub enum Algorithm {
    Binary,
    Sidewinder,
    AldousBroder,
}

impl FromStr for Algorithm {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "binary" => Ok(Algorithm::Binary),
            "sidewinder" => Ok(Algorithm::Sidewinder),
            "aldous-broder" => Ok(Algorithm::AldousBroder),
            _ => Err("no match"),
        }
    }
}

pub enum Output {
    Ascii,
    Svg,
}

impl FromStr for Output {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Output::Ascii),
            "svg" => Ok(Output::Svg),
            _ => Err("no match"),
        }
    }
}
