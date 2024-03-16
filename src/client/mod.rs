use crate::game::Game;

pub struct Client<T:Game, U:AI<T>> {
    game: T,
    ai: U,
}

pub trait AI<T:Game> {
    fn get_next_move(self: Self, state: T) -> T::Move;
}