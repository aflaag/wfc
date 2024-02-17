use crate::tile::Tile;

use std::{collections::HashSet, fmt, hash::Hash};

use rand::{seq::IteratorRandom, Rng};

/// Represents the order of which
/// the neighbours of a tile are going to be visited.
const DIRECTIONS_ORDER: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

/// Represents a direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a wave state.
#[derive(Debug, Clone)]
pub struct Wave<T: Tile + Hash> {
    width: usize,
    height: usize,
    tiles: Vec<Vec<(Option<T>, usize)>>,
    variants_total: usize,
    rules: HashSet<(T, T, Direction)>,
}

impl<T: Tile + Hash> Wave<T> {
    /// Returns a new wave.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wfc::wave::Wave;
    /// # use std::{fmt, collections::HashSet};
    /// # use strum_macros::EnumIter;
    /// use wfc::tile::Tile;
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
    /// pub enum SimpleTile {
    ///     Empty,
    ///     Filled,
    /// }
    ///
    /// impl Tile for SimpleTile {}
    ///
    /// let wave = Wave::<SimpleTile>::new(10, 10, HashSet::new()).unwrap();
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        rules: HashSet<(T, T, Direction)>,
    ) -> Result<Self, WaveError> {
        if width == 0 || height == 0 {
            return Err(WaveError::ZeroDimension);
        }

        let variants_total = T::iter().len();

        Ok(Self {
            width,
            height,
            tiles: (0..height)
                .map(|_| (0..width).map(|_| (None, variants_total)).collect())
                .collect(),
            variants_total,
            rules,
        })
    }

    /// Adds a new rule, where `(first_tile, second_tile, direction)` indicates that
    /// `first_tile` can be placed next to `second_tile` in the `direction` direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wfc::{tile::Tile, wave::{Wave, Direction}};
    /// # use std::{fmt, collections::HashSet};
    /// # use strum_macros::EnumIter;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
    /// # pub enum SimpleTile {
    /// #     Empty,
    /// #     Filled,
    /// # }
    /// # impl Tile for SimpleTile {}
    /// let mut wave = Wave::<SimpleTile>::new(10, 10, HashSet::new()).unwrap();
    ///
    /// wave.add_rule((SimpleTile::Empty, SimpleTile::Filled, Direction::Right)); // now a filled tile can be on the right of an empty tile
    /// ```
    pub fn add_rule(&mut self, rule: (T, T, Direction)) {
        let _ = self.rules.insert(rule);
    }

    /// Removes a new rule, where `(first_tile, second_tile, direction)` indicates that
    /// `first_tile` can be placed next to `second_tile` in the `direction` direction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wfc::{tile::Tile, wave::{Wave, Direction}};
    /// # use std::{fmt, collections::HashSet};
    /// # use strum_macros::EnumIter;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
    /// # pub enum SimpleTile {
    /// #     Empty,
    /// #     Filled,
    /// # }
    /// # impl Tile for SimpleTile {}
    /// let mut wave = Wave::<SimpleTile>::new(10, 10, HashSet::new()).unwrap();
    ///
    /// wave.remove_rule((SimpleTile::Empty, SimpleTile::Filled, Direction::Right)); // now a filled tile can't be on the right of an empty tile anymore
    /// ```
    pub fn remove_rule(&mut self, rule: (T, T, Direction)) {
        let _ = self.rules.remove(&rule);
    }

    /// Returns the following two arrays:
    ///   - the first is a `[bool; 4]` array in which the _i_-th element
    ///   of the array is `true` if and only if the corresponding
    ///   (following the order defined by [`DIRECTIONS_ORDER`]) neighbour exists;
    ///   - the second is an `[Option<T>; 4]` array in which the _i_-th element
    ///   of the array is `None` if the corresponding (following the order defined
    ///   by [`DIRECTIONS_ORDER`]) neighbour either does not exists or it's `None`
    ///   because it hasn't collapsed yet.
    fn neighbours_info(&self, (x, y): (usize, usize)) -> ([bool; 4], [Option<T>; 4]) {
        let mut availables = [false; 4];

        let mut neighbours = [None; 4];

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

    /// Returns `true` if and only if the given tile variant
    /// can be placed next to it's existing collapsed neighbours.
    fn is_valid_tile_variant(&self, tile_variant: T, neighbours: [Option<T>; 4]) -> bool {
        neighbours
            .iter()
            .zip(DIRECTIONS_ORDER)
            .all(|(n, d)| n.map_or(true, |v| self.rules.get(&(tile_variant, v, d)).is_some()))
    }

    /// Updates the tile in the given coordinates, based on its neighbours.
    fn update_tile<R: Rng + ?Sized + Clone>(
        &mut self,
        (x, y): (usize, usize),
        rng: &mut R,
    ) -> Result<(), WaveError> {
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
                    .filter(|tile_variant| self.is_valid_tile_variant(*tile_variant, neighbours))
                    .choose(rng);

                if choice.is_some() {
                    choice
                } else {
                    return Err(WaveError::UncollapsibleWave);
                }
            },
            0,
        );

        [
            (Some(x), y.checked_sub(1)),
            (Some(x), Some(y + 1)),
            (x.checked_sub(1), Some(y)),
            (Some(x + 1), Some(y)),
        ]
        .iter()
        .zip(availables)
        .zip(neighbours)
        .filter(|((_, a), n)| *a && n.is_none())
        .for_each(|(((x, y), _), _)| {
            let (n_x, n_y) = (x.unwrap(), y.unwrap());

            let (_, neighbours) = self.neighbours_info((n_x, n_y));

            self.tiles[n_y][n_x].1 = T::iter()
                .filter(|tile_variant| self.is_valid_tile_variant(*tile_variant, neighbours))
                .count();
        });

        Ok(())
    }

    /// Collapses the wave, using the Wave Function Collapse algorithm.
    ///
    /// # Examples
    ///
    /// ```
    /// # use wfc::{tile::Tile, wave::{Wave, Direction}};
    /// # use std::{fmt, collections::HashSet};
    /// # use strum_macros::EnumIter;
    /// # use rand::thread_rng;
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, Hash)]
    /// # pub enum SimpleTile {
    /// #     Empty,
    /// #     Filled,
    /// # }
    /// # impl Tile for SimpleTile {}
    /// let mut rng = thread_rng();
    ///
    /// let mut wave = Wave::<SimpleTile>::new(10, 10, HashSet::new()).unwrap();
    ///
    /// wave.add_rule((SimpleTile::Empty, SimpleTile::Filled, Direction::Right));
    ///
    /// let outcome = wave.collapse(&mut rng);
    /// ```
    pub fn collapse<R: Rng + ?Sized + Clone>(&mut self, rng: &mut R) -> Result<(), WaveError> {
        let mut collapsed = 0;

        let total_tiles = self.width * self.height;

        while collapsed < total_tiles {
            let lowest_entropy = self
                .tiles
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|tile| tile.1)
                        .filter(|entropy| *entropy != 0)
                        .min()
                        .unwrap_or(self.variants_total)
                })
                .min()
                .unwrap();

            let tile_coords = self
                .tiles
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(_, tile)| tile.1 == lowest_entropy)
                        .map(move |(x, _)| (x, y))
                })
                .choose(rng);

            if tile_coords.is_none() || self.update_tile(tile_coords.unwrap(), rng).is_err() {
                return Err(WaveError::NotFullyCollapsed);
            }

            collapsed += 1;
        }

        Ok(())
    }
}

impl<T: Tile + Hash + fmt::Display> fmt::Display for Wave<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tiles.iter().try_for_each(|row| {
            row.iter().try_for_each(|tile| {
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

/// Represents possible errors for the [`Wave`] structure.
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
