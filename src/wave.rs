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
pub struct Wave<T: Tile> {
    width: usize,
    height: usize,
    tiles: Vec<Vec<(Option<T>, usize)>>,
    variants_total: usize,
    rules: Vec<(T, T, Direction)>,
}

impl<T: Tile> Wave<T> {
    pub fn new(width: usize, height: usize, rules: Vec<(T, T, Direction)>) -> Result<Self, WaveError> {
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
        })
    }

    pub fn add_rule(&mut self, rule: (T, T, Direction)) {
        self.rules.push(rule)
    }

    fn neighbours_info(&self, (x, y): (usize, usize)) -> (bool, bool, bool, bool, Option<T>, Option<T>, Option<T>, Option<T>) {
        let mut top_available = false;
        let mut bottom_available = false;
        let mut left_available = false;
        let mut right_available = false;

        let mut top = None;
        let mut bottom = None;
        let mut left = None;
        let mut right = None;

        if let Some(diff) = y.checked_sub(1) {
            top_available = true;

            if self.tiles[diff][x].0.is_some() {
                top = self.tiles[diff][x].0;
            }
        }

        if let Some(bottom_row) = self.tiles.get(y + 1) {
            bottom_available = true;

            if bottom_row[x].0.is_some() {
                bottom = bottom_row[x].0;
            }
        }

        if let Some(diff) = x.checked_sub(1) {
            left_available = true;

            if self.tiles[y][diff].0.is_some() {
                left = self.tiles[y][diff].0;
            }
        }

        if let Some(right_tile) = self.tiles.get(y).unwrap().get(x + 1) {
            right_available = true;

            if right_tile.0.is_some() {
                right = right_tile.0;
            }
        }

        (
            top_available,
            bottom_available,
            left_available,
            right_available,
            top,
            bottom,
            left,
            right
        )
    }

    fn update_neighbour_entropy(&mut self, (n_x, n_y): (usize, usize)) {
        let (_, _, _, _, top, bottom, left, right) = self.neighbours_info((n_x, n_y));

        self.tiles[n_y][n_x].1 = T::iter()
            .filter(|tile_variant| {
                // TODO: ricontrolla che questa cosa sia giusta
                (top.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, top.unwrap(), Direction::Up))) &&
                (bottom.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, bottom.unwrap(), Direction::Down))) &&
                (left.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, left.unwrap(), Direction::Left))) &&
                (right.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, right.unwrap(), Direction::Right)))
            })
            .count();
    }

    fn choose_tile<R: Rng + ?Sized + Clone>(&mut self, (x, y): (usize, usize), rng: &mut R) -> Result<(), WaveError> {
        let (top_available, bottom_available, left_available, right_available, top, bottom, left, right) = self.neighbours_info((x, y));

        // TODO: check telegram
        self.tiles[y][x] = (
            if (!top_available || top.is_none()) &&
                (!bottom_available || bottom.is_none()) &&
                (!left_available || left.is_none()) &&
                (!right_available || right.is_none())
            {
                T::iter().choose(rng)
            } else {
                let choice = T::iter()
                    .filter(|tile_variant| {
                        (top.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, top.unwrap(), Direction::Up))) &&
                        (bottom.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, bottom.unwrap(), Direction::Down))) &&
                        (left.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, left.unwrap(), Direction::Left))) &&
                        (right.is_none() || self.rules.iter().any(|&r| r == (*tile_variant, right.unwrap(), Direction::Right)))
                    })
                    .choose(rng);

                if choice.is_some() {
                    choice
                } else {
                    return Err(WaveError::UncollapsibleWave);
                }
            }
            ,
            0
        );

        if top_available && top.is_none() {
            self.update_neighbour_entropy((x, y - 1));
        }

        if bottom_available && bottom.is_none() {
            self.update_neighbour_entropy((x, y + 1));
        }

        if left_available && left.is_none() {
            self.update_neighbour_entropy((x - 1, y));
        }

        if right_available && right.is_none() {
            self.update_neighbour_entropy((x + 1, y));
        }

        Ok(())
    }

    // TODO: REMOVE THIS AND FIX THE PRINT
    pub fn tiles(&self) -> Vec<Vec<(Option<T>, usize)>> {
        self.tiles.clone()
    }

    pub fn collapse<R: Rng + ?Sized + Clone>(&mut self, rng: &mut R) -> Result<(), WaveError> {
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
                .choose(rng)
                .unwrap(); // TODO: check

            // println!("{:?}", (tile_x, tile_y));

            if self.choose_tile((tile_x, tile_y), rng).is_ok() {
                collapsed += 1;
            } else {
                // println!("exploded");
                return Err(WaveError::NotFullyCollapsed);
            }

            // print!("---\n");
        }
        
        Ok(())
    }
}

impl<T: Tile> fmt::Display for Wave<T> {
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
