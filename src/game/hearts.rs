use crate::game::Game;

pub struct Hearts {}


impl Hearts {
    pub fn init() -> Self {
        Hearts{}
    }
}

impl Game for Hearts {
    type Move = u8;

    fn game_identifier() -> [u8; 6] {
        ['h' as u8, 'e' as u8, 'a' as u8, 'r' as u8, 't' as u8, 's' as u8]
    }
}