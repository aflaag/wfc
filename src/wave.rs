use crate::tile::Tile;

use std::{collections::HashSet, fmt, hash::Hash};

use rand::{Rng, seq::IteratorRandom};

const DIRECTIONS_ORDER: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Wave<T: Tile + Hash> {
    width: usize,
    height: usize,
    tiles: Vec<Vec<(Option<T>, usize)>>,
    variants_total: usize,
    rules: HashSet<(T, T, Direction)>,
}

impl<T: Tile + Hash> Wave<T> {
    pub fn new(width: usize, height: usize, rules: HashSet<(T, T, Direction)>) -> Result<Self, WaveError> {
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
        let _ = self.rules.insert(rule);
    }

    pub fn remove_rule(&mut self, rule: (T, T, Direction)) {
        let _ = self.rules.remove(&rule);
    }

    fn neighbours_info(&self, (x, y): (usize, usize)) -> ([bool; 4], [Option<T>; 4]) {
        let mut availables = [false; 4];

        let mut neighbours = [None; 4];

        // TODO: pensa se ci sta un modo migliore di scrivere sta cosa
        if let Some(diff) = y.checked_sub(1) {
            availables[0] = true;

            neighbours[0] = self.tiles[diff][x].0;
        }

        if let Some(bottom_row) = self.tiles.get(y + 1) {
            availables[1] = true;

            neighbours[1] = bottom_row[x].0;
        }

        if let Some(diff) = x.checked_sub(1) {
            availables[2] = true;

            neighbours[2] = self.tiles[y][diff].0;
        }

        if let Some(right_tile) = self.tiles.get(y).unwrap().get(x + 1) {
            availables[3] = true;

            neighbours[3] = right_tile.0;
        }

        (availables, neighbours)
    }

    fn update_neighbour_entropy(&mut self, (n_x, n_y): (usize, usize)) {
        let (_, neighbours) = self.neighbours_info((n_x, n_y));

        self.tiles[n_y][n_x].1 = T::iter()
            .filter(|tile_variant| {
                neighbours
                    .iter()
                    .zip(DIRECTIONS_ORDER)
                    .all(|(n, d)| n.map_or(true, |v| self.rules.get(&(*tile_variant, v, d)).is_some()))
            })
            .count();
    }

    fn choose_tile<R: Rng + ?Sized + Clone>(&mut self, (x, y): (usize, usize), rng: &mut R) -> Result<(), WaveError> {
        let (availables, neighbours) = self.neighbours_info((x, y));

        self.tiles[y][x] = (
            if availables
                .iter()
                .zip(neighbours)
                .all(|(a, n)| !a || n.is_none())
            {
                T::iter().choose(rng)
            } else {
                let choice = T::iter()
                    .filter(|tile_variant| {
                        neighbours
                            .iter()
                            .zip(DIRECTIONS_ORDER)
                            .all(|(n, d)| n.map_or(true, |v| self.rules.get(&(*tile_variant, v, d)).is_some()))
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

        [
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y))
        ]
            .iter()
            .zip(availables)
            .zip(neighbours)
            .filter(|((_, a), n)| *a && n.is_none())
            .for_each(|(((x, y), _), _)| self.update_neighbour_entropy((x.unwrap(), y.unwrap())));

        Ok(())
    }

    pub fn collapse<R: Rng + ?Sized + Clone>(&mut self, rng: &mut R) -> Result<(), WaveError> {
        let mut collapsed = 0;

        let total_tiles = self.width * self.height;

        while collapsed < total_tiles {
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
                .unwrap();

            let tile_coords = self
                .tiles
                .iter()
                .enumerate()
                .flat_map(|(y, row)| row
                     .iter()
                     .enumerate()
                     .filter(|(_, tile)| tile.1 == lowest_entropy)
                     .map(move |(x, _)| (x, y))
                )
                .choose(rng);

            if tile_coords.is_none() || self.choose_tile(tile_coords.unwrap(), rng).is_err() {
                return Err(WaveError::NotFullyCollapsed);
            }

            collapsed += 1;
        }
        
        Ok(())
    }
}

impl<T: Tile + Hash> fmt::Display for Wave<T> {
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
                    })?;
                writeln!(f)
            })
    }
}

#[derive(Debug)]
pub enum WaveError {
    ZeroDimension,
    NotFullyCollapsed,
    UncollapsibleWave,
}

impl std::fmt::Display for WaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::ZeroDimension => writeln!(f, "Dimensions can't be 0."),
            Self::NotFullyCollapsed => writeln!(f, "The wave has not fully collapsed."),
            Self::UncollapsibleWave => writeln!(f, "Tha wave can't be collapsed any further."),
        }
    }
}

impl std::error::Error for WaveError {}
