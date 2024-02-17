use std::{fmt, hash::Hash};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Represents a wave tile.
pub trait Tile: PartialEq + Eq + Clone + Copy + IntoEnumIterator + Hash {}

/// Reperesents a tile of a Maze.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
pub enum MazeTile {
    /// Represents `┳`
    TShaped,

    /// Represents `┫`
    TShapedLeft,

    /// Represents `┣`
    TShapedRight,

    /// Represents `┻`
    TShapedUpsideDown,

    /// Represents `┓`
    TopRightCorner,

    /// Represents `┏`
    TopLeftCorner,

    /// Represents `┗`
    BottomLeftCorner,

    /// Represents `┛`
    BottomRightCorner,

    /// Represents `━`
    HorizontalLine,

    /// Represents `┃`
    VerticalLine,

    /// Represents `╋`
    CenterCross,
    
    /// Represents ` `
    Empty
}

impl Tile for MazeTile {}

impl fmt::Display for MazeTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::TShaped => write!(f, "┳"),
            Self::TShapedLeft => write!(f, "┫"),
            Self::TShapedRight => write!(f, "┣"),
            Self::TShapedUpsideDown => write!(f, "┻"),
            Self::TopRightCorner => write!(f, "┓"),
            Self::TopLeftCorner => write!(f, "┏"),
            Self::BottomLeftCorner => write!(f, "┗"),
            Self::BottomRightCorner => write!(f, "┛"),
            Self::HorizontalLine => write!(f, "━"),
            Self::VerticalLine => write!(f, "┃"),
            Self::CenterCross => write!(f, "╋"),
            Self::Empty => write!(f, " "),
        }
    }
}
