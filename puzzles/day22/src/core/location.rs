use anyhow::Context;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Location {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Location {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    pub fn above(&self) -> Self {
        Self::new(self.x, self.y, self.z + 1)
    }

    pub fn below(&self) -> Self {
        Self::new(self.x, self.y, self.z - 1)
    }
}

type Tuple = (usize, usize, usize);

impl From<Location> for Tuple {
    fn from(Location { x, y, z }: Location) -> Self {
        (x, y, z)
    }
}

impl From<Tuple> for Location {
    fn from((x, y, z): Tuple) -> Self {
        Self::new(x, y, z)
    }
}

impl std::convert::TryFrom<&str> for Location {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut tokens = input.splitn(3, ',');

        let x = tokens.next().context("missing x")?.parse().context("x")?;
        let y = tokens.next().context("missing y")?.parse().context("y")?;
        let z = tokens.next().context("missing z")?.parse().context("z")?;

        Ok(Self::new(x, y, z))
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
