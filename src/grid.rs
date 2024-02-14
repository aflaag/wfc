use crate::tile::Tile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Grid<T: Tile> {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Option<T>>>,
    rules: Vec<(T, T, Direction)>,
}

impl<T: Tile> Grid<T> {
    pub fn new(width: usize, height: usize, rules: Vec<(T, T, Direction)>) -> Self {
        Self {
            width,
            height,
            tiles: (0..height).map(|_| (0..width).map(|_| None).collect()).collect(),
            rules,
        }
    }

    pub fn add_rule(&mut self, rule: (T, T, Direction)) {
        self.rules.push(rule)
    }

    fn entropy(&self, (x, y): (usize, usize)) -> f32 {
        if let Some(tile) = self.tiles[y][x] {
            let mut entropy = 0.0;

            if x < self.width {
                if let Some(right_neighbour) = self.tiles[y][x + 1] {
                    if self.rules.iter().any(|&r| r == (tile, right_neighbour, Direction::Right)) {
                        entropy += 1.0
                    }
                }
            }

            entropy
        } else {
            f32::INFINITY
        }
    }
}
