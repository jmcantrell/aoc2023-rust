use anyhow::{anyhow, ensure};

use Category::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl std::convert::TryFrom<char> for Category {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'x' => Ok(ExtremelyCoolLooking),
            'm' => Ok(Musical),
            'a' => Ok(Aerodynamic),
            's' => Ok(Shiny),
            _ => Err(anyhow!("invalid category: {:?}", input)),
        }
    }
}

impl std::convert::TryFrom<&str> for Category {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        ensure!(
            input.len() == 1,
            "expected input to be a single character, but it was {} characters",
            input.len()
        );

        input.chars().next().unwrap().try_into()
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExtremelyCoolLooking => "extremely cool looking",
                Musical => "musical",
                Aerodynamic => "aerodynamic",
                Shiny => "shiny",
            }
        )
    }
}
