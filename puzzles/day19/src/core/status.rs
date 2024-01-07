use anyhow::{anyhow, ensure};

use Status::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Accepted,
    Rejected,
}

impl std::convert::TryFrom<char> for Status {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'A' => Ok(Accepted),
            'R' => Ok(Rejected),
            _ => Err(anyhow!("invalid status: {:?}", input)),
        }
    }
}

impl std::convert::TryFrom<&str> for Status {
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
