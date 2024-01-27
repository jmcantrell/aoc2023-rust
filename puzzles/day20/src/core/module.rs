#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ModuleKind {
    #[default]
    Broadcast,
    FlipFlop,
    Conjunction,
}
