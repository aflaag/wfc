use wfc::{wave::Wave, tile::MazeTile};
use rand::thread_rng;

fn main() {
    let rng = thread_rng();

    let mut wave = Wave::<_, MazeTile>::new(10, 10, vec![], rng).unwrap();

    if wave.collapse().is_ok() {
        println!("{}", wave);
    }
}
