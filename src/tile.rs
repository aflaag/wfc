use std::fmt;

pub trait Tile: PartialEq + Eq + Clone + Copy + fmt::Display {}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum TilePosition {
//     Center,
//     Top,
//     Bottom,
//     Left,
//     Right,
//     TopLeft,
//     TopRight,
//     BottomLeft,
//     BottomRight,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MazeTile {
    /// Represents `╦`
    TShaped,

    /// Represents `╣`
    TShapedLeft,

    /// Represents `╠`
    TShapedRight,

    /// Represents `╩`
    TShapedUpsideDown,

    /// Represents `╗`
    TopRightCorner,

    /// Represents `╔`
    TopLeftCorner,

    /// Represents `╚`
    BottoLeftCorner,

    /// Represents `╝`
    BottomRightCorner,

    /// Represents `═`
    HorizontalLine,

    /// Represents `║`
    VerticalLine,

    /// Represents `╬`
    CenterCross,
}

impl Tile for MazeTile {}

impl fmt::Display for MazeTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TShaped => write!(f, "╦"),
            Self::TShapedLeft => write!(f, "╣"),
            Self::TShapedRight => write!(f, "╠"),
            Self::TShapedUpsideDown => write!(f, "╩"),
            Self::TopRightCorner => write!(f, "╗"),
            Self::TopLeftCorner => write!(f, "╔"),
            Self::BottoLeftCorner => write!(f, "╚"),
            Self::BottomRightCorner => write!(f, "╝"),
            Self::HorizontalLine => write!(f, "═"),
            Self::VerticalLine => write!(f, "║"),
            Self::CenterCross => write!(f, "╬"),
        }
    }
}
