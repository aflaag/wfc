use wfc::{tile::MazeTile, wave::{Direction, Wave}};

use std::collections::HashSet;

use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let mut wave = Wave::<MazeTile>::new(80, 80, HashSet::new()).unwrap();

    // rules for `┳`
    wave.add_rule((MazeTile::TShaped, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::HorizontalLine, Direction::Up));
    wave.add_rule((MazeTile::TShaped, MazeTile::Empty, Direction::Up));

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

    // rules for `┫`
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TShapedRight, Direction::Right));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::TopLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::VerticalLine, Direction::Right));
    wave.add_rule((MazeTile::TShapedLeft, MazeTile::Empty, Direction::Right));

    // rules for `┣`
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedLeft, Direction::Left));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TopRightCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::BottomRightCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::VerticalLine, Direction::Left));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::Empty, Direction::Left));

    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::TShapedRight, MazeTile::CenterCross, Direction::Right));

    // rules for `┻`
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShaped, Direction::Down));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::HorizontalLine, Direction::Down));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::Empty, Direction::Down));

    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::HorizontalLine, Direction::Left));
    
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::TShapedUpsideDown, MazeTile::CenterCross, Direction::Right));

    // rules for `┓`
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::HorizontalLine, Direction::Up));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::Empty, Direction::Up));

    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TShapedRight, Direction::Right));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::TopLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::TopRightCorner, MazeTile::VerticalLine, Direction::Right));

    // rules for `┏`
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::HorizontalLine, Direction::Up));
    wave.add_rule((MazeTile::TopLeftCorner, MazeTile::Empty, Direction::Up));

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

    // rules for `┗`
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShaped, Direction::Down));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopRightCorner, Direction::Down));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::HorizontalLine, Direction::Down));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::Empty, Direction::Down));

    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShapedLeft, Direction::Left));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopRightCorner, Direction::Left));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::BottomRightCorner, Direction::Left));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::VerticalLine, Direction::Left));

    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::BottomLeftCorner, MazeTile::CenterCross, Direction::Right));

    // rules for `┛`
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShaped, Direction::Down));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopRightCorner, Direction::Down));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::HorizontalLine, Direction::Down));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::Empty, Direction::Down));

    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TShapedRight, Direction::Right));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::TopLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::BottomRightCorner, MazeTile::VerticalLine, Direction::Right));

    // rules for `━`
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::HorizontalLine, Direction::Up));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::Empty, Direction::Up));

    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShaped, Direction::Down));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TopRightCorner, Direction::Down));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::HorizontalLine, Direction::Down));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::Empty, Direction::Down));

    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::HorizontalLine, MazeTile::CenterCross, Direction::Right));

    // rules for `┃`
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedLeft, Direction::Left));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TopRightCorner, Direction::Left));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::BottomRightCorner, Direction::Left));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::VerticalLine, Direction::Left));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::Empty, Direction::Left));

    wave.add_rule((MazeTile::VerticalLine, MazeTile::TShapedRight, Direction::Right));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::TopLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::VerticalLine, Direction::Right));
    wave.add_rule((MazeTile::VerticalLine, MazeTile::Empty, Direction::Right));

    // rules for `╋`
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShaped, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedLeft, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedRight, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TopRightCorner, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TopLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::VerticalLine, Direction::Up));
    wave.add_rule((MazeTile::CenterCross, MazeTile::CenterCross, Direction::Up));

    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedLeft, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedRight, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedUpsideDown, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::BottomLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::BottomRightCorner, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::VerticalLine, Direction::Down));
    wave.add_rule((MazeTile::CenterCross, MazeTile::CenterCross, Direction::Down));

    wave.add_rule((MazeTile::CenterCross, MazeTile::TShaped, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedRight, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedUpsideDown, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TopLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::BottomLeftCorner, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::HorizontalLine, Direction::Left));
    wave.add_rule((MazeTile::CenterCross, MazeTile::CenterCross, Direction::Left));

    wave.add_rule((MazeTile::CenterCross, MazeTile::TShaped, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedLeft, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TShapedUpsideDown, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::TopRightCorner, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::BottomRightCorner, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::HorizontalLine, Direction::Right));
    wave.add_rule((MazeTile::CenterCross, MazeTile::CenterCross, Direction::Right));

    // rules for ` `
    wave.add_rule((MazeTile::Empty, MazeTile::TShapedUpsideDown, Direction::Up));
    wave.add_rule((MazeTile::Empty, MazeTile::BottomLeftCorner, Direction::Up));
    wave.add_rule((MazeTile::Empty, MazeTile::BottomRightCorner, Direction::Up));
    wave.add_rule((MazeTile::Empty, MazeTile::HorizontalLine, Direction::Up));
    wave.add_rule((MazeTile::Empty, MazeTile::Empty, Direction::Up));

    wave.add_rule((MazeTile::Empty, MazeTile::TShaped, Direction::Down));
    wave.add_rule((MazeTile::Empty, MazeTile::TopRightCorner, Direction::Down));
    wave.add_rule((MazeTile::Empty, MazeTile::TopLeftCorner, Direction::Down));
    wave.add_rule((MazeTile::Empty, MazeTile::HorizontalLine, Direction::Down));
    wave.add_rule((MazeTile::Empty, MazeTile::Empty, Direction::Down));

    wave.add_rule((MazeTile::Empty, MazeTile::TShapedLeft, Direction::Left));
    wave.add_rule((MazeTile::Empty, MazeTile::TopRightCorner, Direction::Left));
    wave.add_rule((MazeTile::Empty, MazeTile::BottomRightCorner, Direction::Left));
    wave.add_rule((MazeTile::Empty, MazeTile::VerticalLine, Direction::Left));
    wave.add_rule((MazeTile::Empty, MazeTile::Empty, Direction::Left));

    wave.add_rule((MazeTile::Empty, MazeTile::TShapedRight, Direction::Right));
    wave.add_rule((MazeTile::Empty, MazeTile::TopLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::Empty, MazeTile::BottomLeftCorner, Direction::Right));
    wave.add_rule((MazeTile::Empty, MazeTile::VerticalLine, Direction::Right));
    wave.add_rule((MazeTile::Empty, MazeTile::Empty, Direction::Right));

    let _ = wave.collapse(&mut rng);

    println!("{}", wave);
}
