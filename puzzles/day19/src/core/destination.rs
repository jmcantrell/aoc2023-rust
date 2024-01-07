use super::Status;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Destination<'a> {
    Workflow(&'a str),
    Status(Status),
}

impl Destination<'_> {
    pub fn workflow(&self) -> Option<&str> {
        match self {
            Self::Workflow(label) => Some(label),
            _ => None,
        }
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Destination<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        if let Ok(status) = Status::try_from(input) {
            Ok(Self::Status(status))
        } else {
            Ok(Self::Workflow(input))
        }
    }
}
