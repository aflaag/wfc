use wfc::{wave::Wave, tile::MazeTile};
use rand::thread_rng;

fn main() {
    let rng = thread_rng();

    let mut wave = Wave::<_, MazeTile>::new(10, 10, vec![], rng).unwrap();

    let _ = wave.collapse();

    println!("{}", wave);
}
