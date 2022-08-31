#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinoTypes {
    I,
    L,
    J,
    Z,
    S,
    O,
    T,
    Unknown,
}

pub const ALL_MINO_TYPES: Vec<MinoTypes> = vec![
    MinoTypes::I,
    MinoTypes::L,
    MinoTypes::J,
    MinoTypes::Z,
    MinoTypes::S,
    MinoTypes::O,
    MinoTypes::T,
]
