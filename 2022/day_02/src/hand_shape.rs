//

use std::convert::Infallible;

//

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl std::str::FromStr for HandShape {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => unreachable!("invalid hand shape : {s}"),
        }
    }
}
