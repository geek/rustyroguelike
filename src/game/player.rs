use crate::rltk;
use rltk::Color;
use rltk::Point;

pub struct Player {
    pub position : Point,
    pub glyph: u8,
    pub fg : Color,
    pub visible_tiles : Vec<Point>
}

impl Player {
    pub fn new(x:i32, y:i32, glyph:u8, fg : Color) -> Player {
        Player{ position: Point::new(x, y), glyph, fg, visible_tiles: Vec::new() }
    }
}