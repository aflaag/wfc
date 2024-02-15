use wfc::{tile::MazeTile, wave::{Direction, Wave}};
use rand::thread_rng;

fn main() {
    let rng = thread_rng();

    let mut wave = Wave::<_, MazeTile>::new(10, 10, vec![], rng).unwrap();

    // rules for `╦`
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::HorizontalLine, Direction::Up));

    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::TShaped, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::TShaped, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::TShaped, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::TShaped, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::TShaped, MazeTile::CenterCross, Direction::Right));

    // rules for `╣`

    // rules for `╠`

    // rules for `╩`

    // rules for `╗`

    // rules for ╔
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::HorizontalLine, Direction::Up));

    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedLeft, Direction::Left));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TopRightCorner, Direction::Left));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomRightCorner, Direction::Left));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::VerticalLine, Direction::Left));

    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::CenterCross, Direction::Right));

    // rules for `╚`

    // rules for `╝`

    // rules for `═`

    // rules for `║`

    // rules for `╬`

    let _ = wave.collapse();

    println!("{}", wave);
}
