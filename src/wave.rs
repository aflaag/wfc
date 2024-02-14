use crate::tile::{Tile};

use rand::{Rng, seq::IteratorRandom};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Wave<R: Rng + ?Sized + Clone, T: Tile> {
    width: usize,
    height: usize,
    tiles: Vec<Vec<(Option<T>, usize)>>,
    rules: Vec<(T, T, Direction)>,
    rng: R,
}

impl<R: Rng + ?Sized + Clone, T: Tile> Wave<R, T> {
    pub fn new(width: usize, height: usize, rules: Vec<(T, T, Direction)>, rng: R) -> Result<Self, WaveError> {
        if width == 0 || height == 0 {
            return Err(WaveError::ZeroDimension)
        }

        Ok(Self {
            width,
            height,
            tiles: (0..height).map(|_| (0..width).map(|_| (None, usize::MAX)).collect()).collect(),
            rules,
            rng,
        })
    }

    pub fn add_rule(&mut self, rule: (T, T, Direction)) {
        self.rules.push(rule)
    }

    // fn get_neighbours(&self, (x, y): (usize, usize)) -> TilePosition {
    //     if x == self.width - 1 {
    //         if y == self.width - 1 {
    //             TilePosition::BottomRight
    //         } else {
    //             TilePosition
    //         }
    //     }
    // }

    pub fn collapse(&mut self) -> Result<(), WaveError> {
        let mut collapsed = 0;

        while collapsed < self.width * self.height {
            let lowest_entropy = self
                .tiles
                .iter()
                .map(|row| row
                     .iter()
                     .map(|tile| tile.1)
                     .min()
                     .unwrap() // TODO: check
                )
                .min()
                .unwrap(); // TODO: check

            let (tile_x, tile_y) = self
                .tiles
                .iter()
                .enumerate()
                .map(|(y, row)| row
                     .iter()
                     .enumerate()
                     .filter(|(_, tile)| tile.1 == lowest_entropy)
                     .map(move |(x, _)| (x, y))
                )
                .flatten()
                .choose(&mut self.rng)
                .unwrap(); // TODO: check

            println!("{:?}", (tile_x, tile_y));

            // if let Some(upper_row) = self.tiles.get(tile_y - 1) {
            //     if let Some(right_row) = upper_row.get(tile_x + 1) {
            //         if let Some(left_row) = upper_row.get(tile_x - 1) {
            //
            //         }
            //     } else {
            //     }
            // }

            // if x == self.width - 1 {
            //     if y == self.height {
            //         // up
            //         // left
            //     }
            //
            //     // right
            // } else if x == 0 {
            //     if y == 0 {
            //         // right
            //         // down
            //     }
            //
            //     // left
            // }

            // let chosen_tile = (0..self.height)
            //     .map(|y| {
            //         (0..self.width)
            //             .map(|x| )
            //     })

            // let mut lowest_entropy_tiles_count = self
            //     .tiles
            //     .iter()
            //     .map(|row| row
            //          .iter()
            //          .map(|tile| )
            //          .fold(0, |acc, x| acc + x)
            //      )
            //     .fold(0, |acc, x| acc + x);
        }
        
        Ok(())
    }

    // fn entropy(&self, (x, y): (usize, usize)) -> f32 {
    //     if let Some(tile) = self.tiles[y][x].0 {
    //         let mut entropy = 0.0;
    //
    //         if x < self.width {
    //             if let Some(right_neighbour) = self.tiles[y][x + 1].0 {
    //                 if self.rules.iter().any(|&r| r == (tile, right_neighbour, Direction::Right)) {
    //                     entropy += 1.0
    //                 }
    //             }
    //         }
    //
    //         entropy
    //     } else {
    //         f32::INFINITY
    //     }
    // }
}

impl<R: Rng + ?Sized + Clone, T: Tile> fmt::Display for Wave<R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tiles.iter().map(|row| {
            row.iter().map(|tile| {
                if let Some(v) = tile.0 {
                    write!(f, "{}", v)
                } else {
                    write!(f, "X")
                }
            }).collect()
        }).collect()
    }
}

#[derive(Debug)]
pub enum WaveError {
    ZeroDimension,
    NotFullyCollapsed,
}

impl std::fmt::Display for WaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ZeroDimension => writeln!(f, "Dimensions can't be 0."),
            Self::NotFullyCollapsed => writeln!(f, "The wave has not fully collapsed."),
        }
    }
}

impl std::error::Error for WaveError {}
