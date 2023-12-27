use anyhow::{anyhow, ensure, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    Remove,
    Set(usize),
}

impl std::convert::TryFrom<&str> for Operation {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        ensure!(!input.is_empty(), "empty input");

        let (key, rest) = input.split_at(1);

        match key {
            "-" => {
                ensure!(rest.is_empty(), "unexpected trailing characters");
                Ok(Self::Remove)
            }
            "=" => {
                let value = rest.parse().context("value")?;
                Ok(Self::Set(value))
            }
            _ => Err(anyhow!("invalid operation: {:?}", input)),
        }
    }
}
