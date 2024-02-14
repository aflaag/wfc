use crate::tile::Tile;

#[derive(Debug, Clone)]
pub struct Grid<T: Tile> {
    tiles: Vec<Vec<T>>,
    rules: Vec<(T, T)>,
}

impl<T: Tile> Grid<T> {
    pub fn add_rule(&mut self, rule: (T, T)) {
        self.rules.push(rule)
    }
}
