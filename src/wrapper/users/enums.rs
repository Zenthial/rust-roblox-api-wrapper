use std::fmt;

pub enum Limits {
    Ten, TwentyFive, Fifty, OneHundred
}

impl fmt::Display for Limits {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Limits::Ten => write!(formatter, "10"),
            Limits::TwentyFive => write!(formatter, "25"),
            Limits::Fifty => write!(formatter, "50"),
            Limits::OneHundred => write!(formatter, "100")
        }
    }
}