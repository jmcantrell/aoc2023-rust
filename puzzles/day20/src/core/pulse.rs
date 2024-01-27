#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PulseKind {
    #[default]
    Low,
    High,
}

impl std::fmt::Display for PulseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Low => "low",
                Self::High => "high",
            }
        )
    }
}
