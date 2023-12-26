use anyhow::anyhow;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Tile {
    #[default]
    Empty,
    RoundRock,
    CubeRock,
}

impl std::convert::TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::RoundRock),
            '#' => Ok(Self::CubeRock),
            _ => Err(anyhow!("invalid tile: {:?}", input)),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::RoundRock => 'O',
                Self::CubeRock => '#',
            }
        )
    }
}
