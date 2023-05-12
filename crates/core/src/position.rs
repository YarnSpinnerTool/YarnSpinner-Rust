/// Represents a position in a multi-line string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    /// The zero-indexed line of this position.
    pub line: usize,

    /// The zero-indexed character number of this position.
    pub character: usize,
}
