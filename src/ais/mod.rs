pub mod test;

use crate::game::Game;

pub trait AI<T:Game> {
    fn get_next_move(self: &Self, state: &T) -> T::Move;
    fn get_name(self: &Self) -> String;
}