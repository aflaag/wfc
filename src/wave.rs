use crate::tile::Tile;

use std::fmt;

use rand::{Rng, seq::IteratorRandom};
// use bitvec::prelude::*;

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
    variants_total: usize,
    rules: Vec<(T, T, Direction)>,
    rng: R,
}

impl<R: Rng + ?Sized + Clone, T: Tile + std::fmt::Debug> Wave<R, T> {
    pub fn new(width: usize, height: usize, rules: Vec<(T, T, Direction)>, rng: R) -> Result<Self, WaveError> {
        if width == 0 || height == 0 {
            return Err(WaveError::ZeroDimension);
        }

        let variants_total = T::iter().len();

        Ok(Self {
            width,
            height,
            tiles: (0..height).map(|_| (0..width).map(|_| (None, variants_total)).collect()).collect(),
            variants_total,
            rules,
            rng,
        })
    }

    pub fn add_rule(&mut self, rule: (T, T, Direction)) {
        self.rules.push(rule)
    }

    fn choose_tile(&mut self, (x, y): (usize, usize)) -> Result<(), WaveError> {
        let mut none_neighbours = 0;
        let mut neighbours_count = 0;

        let mut top = None;
        let mut bottom = None;
        let mut left = None;
        let mut right = None;

        if let Some(diff) = y.checked_sub(1) {
            neighbours_count += 1;

            if self.tiles[diff][x].0.is_some() {
                top = self.tiles[diff][x].0;
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(bottom_row) = self.tiles.get(y + 1) {
            neighbours_count += 1;

            if bottom_row[x].0.is_some() {
                bottom = bottom_row[x].0;
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(diff) = x.checked_sub(1) {
            neighbours_count += 1;

            if self.tiles[y][diff].0.is_some() {
                left = self.tiles[y][diff].0;
            } else {
                none_neighbours += 1;
            }
        }

        if let Some(right_tile) = self.tiles.get(y).unwrap().get(x + 1) {
            neighbours_count += 1;

            if right_tile.0.is_some() {
                right = right_tile.0;
            } else {
                none_neighbours += 1;
            }
        }

        self.tiles[y][x] = (
            if none_neighbours != neighbours_count {
                let choice = T::iter()
                    .filter(|tile_variant| {
                        (top.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, top.unwrap(), Direction::Up))) &&
                        (bottom.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, bottom.unwrap(), Direction::Down))) &&
                        (left.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, left.unwrap(), Direction::Left))) &&
                        (right.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, right.unwrap(), Direction::Right)))
                    })
                    .choose(&mut self.rng);

                if choice.is_some() {
                    choice
                } else {
                    return Err(WaveError::UncollapsibleWave);
                }
            } else {
                T::iter().choose(&mut self.rng)
            }
            ,
            0
        );

        Ok(())
    }

    // TODO: REMOVE THIS AND FIX THE PRINT
    pub fn tiles(&self) -> Vec<Vec<(Option<T>, usize)>> {
        self.tiles.clone()
    }

    pub fn collapse(&mut self) -> Result<(), WaveError> {
        let mut collapsed = 0;

        let total_tiles = self.width * self.height;

        while collapsed < total_tiles {
            // println!("{}", collapsed);
            // for line in &self.tiles {
            //     for tile in line {
            //         print!("{} ", tile.1);
            //     }
            //
            //     print!("\n");
            // }

            let lowest_entropy = self
                .tiles
                .iter()
                .map(|row| row
                     .iter()
                     .map(|tile| tile.1)
                     .filter(|entropy| *entropy != 0)
                     .min()
                     .unwrap_or(self.variants_total)
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

            if self.choose_tile((tile_x, tile_y)).is_ok() {
                collapsed += 1;
            } else {
                // println!("exploded");
                return Err(WaveError::NotFullyCollapsed);
            }

            print!("---\n");
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
    // UnsupportedVariantsNumber,
    NotFullyCollapsed,
    UncollapsibleWave,
}

impl std::fmt::Display for WaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ZeroDimension => writeln!(f, "Dimensions can't be 0."),
            // Self::UnsupportedVariantsNumber => writeln!(f, "The number of variants of the tiles is too big."),
            Self::NotFullyCollapsed => writeln!(f, "The wave has not fully collapsed."),
            Self::UncollapsibleWave => writeln!(f, "Tha wave can't be collapsed any further."),
        }
    }
}

impl std::error::Error for WaveError {}
