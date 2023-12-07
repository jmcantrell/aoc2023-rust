pub type Digit = u32;
pub type Symbol = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Empty,
    Digit(Digit),
    Symbol(Symbol),
}

impl Space {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    pub fn digit(&self) -> Option<&Digit> {
        match self {
            Self::Digit(digit) => Some(digit),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<&Symbol> {
        match self {
            Self::Symbol(symbol) => Some(symbol),
            _ => None,
        }
    }
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        if let Some(digit) = c.to_digit(10) {
            Self::Digit(digit)
        } else if c == '.' {
            Self::Empty
        } else {
            Self::Symbol(c)
        }
    }
}
