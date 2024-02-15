use crate::tile::Tile;

use std::fmt;

use rand::{Rng, seq::IteratorRandom};
use bitvec::prelude::*;

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
            return Err(WaveError::ZeroDimension);
        }

        if T::iter().len() > usize::MAX {
            return Err(WaveError::UnsupportedVariantsNumber)
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

    fn update_bitmask(options_bitmask: &mut usize, idx: usize) {
        if (*options_bitmask << idx) & 1 == 0 {
            *options_bitmask += if idx == 0 {
                1
            } else {
                2 << (idx - 1)
            }
        }
    }

    fn update_entropy(&mut self, (x, y): (usize, usize)) -> Result<(), WaveError> {
        let mut none_neighbours = 0;
        let mut neighbours = 0;

        let mut options_bitmask: usize = 0;

        if let Some(diff) = y.checked_sub(1) {
            neighbours += 1;

            let top = self.tiles[diff][x];

            if let Some(top_tile) = top.0 {
                T::iter()
                    .enumerate()
                    .for_each(|(idx, tile_variant)| {
                        if self.rules.iter().any(|&r| r == (tile_variant, top_tile, Direction::Right)) {
                            Self::update_bitmask(&mut options_bitmask, idx);
                        }
                    })
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(bottom_row) = self.tiles.get(y + 1) {
            neighbours += 1;

            let bottom = bottom_row[x];

            if let Some(bottom_tile) = bottom.0 {
                T::iter()
                    .enumerate()
                    .for_each(|(idx, tile_variant)| {
                        if self.rules.iter().any(|&r| r == (tile_variant, bottom_tile, Direction::Right)) {
                            Self::update_bitmask(&mut options_bitmask, idx);
                        }
                    })
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(diff) = x.checked_sub(1) {
            neighbours += 1;

            let left = self.tiles[y][diff];

            if let Some(left_tile) = left.0 {
                T::iter()
                    .enumerate()
                    .for_each(|(idx, tile_variant)| {
                        if self.rules.iter().any(|&r| r == (tile_variant, left_tile, Direction::Right)) {
                            Self::update_bitmask(&mut options_bitmask, idx);
                        }
                    })
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(right) = self.tiles.get(y).unwrap().get(x + 1) {
            neighbours += 1;

            if let Some(right_tile) = right.0 {
                T::iter()
                    .enumerate()
                    .for_each(|(idx, tile_variant)| {
                        if self.rules.iter().any(|&r| r == (tile_variant, right_tile, Direction::Right)) {
                            Self::update_bitmask(&mut options_bitmask, idx);
                        }
                    })
            } else {
                none_neighbours += 1;
            }
        }

        self.tiles[y][x] = if none_neighbours == neighbours {
            (T::iter().choose(&mut self.rng), 0)
        } else {
            if let Some(chosen_idx) = <[usize; 1] as Into<BitArray<[usize; 1]>>>::into([options_bitmask])
                .iter()
                .enumerate()
                .filter(|(_, bit)| *bit == true)
                .map(|(idx, _)| idx)
                .take(T::iter().len())
                .choose(&mut self.rng)
            {
                (T::iter().nth(chosen_idx), 0)
            } else {
                println!("did it go here?");
                return Err(WaveError::UncollapsibleWave);
            }
        };

        Ok(())
    }

    // TODO: REMOVE THIS AND FIX THE PRINT
    pub fn tiles(&self) -> Vec<Vec<(Option<T>, usize)>> {
        self.tiles.clone()
    }

    pub fn collapse(&mut self) -> Result<(), WaveError> {
        let mut collapsed = 0;

        while collapsed < self.width * self.height {
            println!("{}", collapsed);
            let lowest_entropy = self
                .tiles
                .iter()
                .map(|row| row
                     .iter()
                     .map(|tile| tile.1)
                     .filter(|entropy| *entropy != 0)
                     .min()
                     .unwrap_or(usize::MAX)
                )
                .min()
                .unwrap(); // TODO: check, shouldn't ever crash theoretically

            let (tile_x, tile_y) = self
                .tiles
                .iter()
                .enumerate()
                .flat_map(|(y, row)| row
                     .iter()
                     .enumerate()
                     .filter(|(_, tile)| tile.1 == lowest_entropy)
                     .map(move |(x, _)| (x, y))
                )
                .choose(&mut self.rng)
                .unwrap(); // TODO: check

            println!("{:?}", (tile_x, tile_y));

            if self.update_entropy((tile_x, tile_y)).is_ok() {
                collapsed += 1;
            } else {
                println!("exploded");
                return Err(WaveError::NotFullyCollapsed);
            }
        }
        
        Ok(())
    }
}

impl<R: Rng + ?Sized + Clone, T: Tile> fmt::Display for Wave<R, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self
            .tiles
            .iter()
            .try_for_each(|row| {
                row
                    .iter()
                    .try_for_each(|tile| {
                        if let Some(v) = tile.0 {
                            write!(f, "{}", v)
                        } else {
                            write!(f, "X")
                        }
                    })
            })
    }
}

#[derive(Debug)]
pub enum WaveError {
    ZeroDimension,
    UnsupportedVariantsNumber,
    NotFullyCollapsed,
    UncollapsibleWave,
}

impl std::fmt::Display for WaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ZeroDimension => writeln!(f, "Dimensions can't be 0."),
            Self::UnsupportedVariantsNumber => writeln!(f, "The number of variants of the tiles is too big."),
            Self::NotFullyCollapsed => writeln!(f, "The wave has not fully collapsed."),
            Self::UncollapsibleWave => writeln!(f, "Tha wave can't be collapsed any further."),
        }
    }
}

impl std::error::Error for WaveError {}
