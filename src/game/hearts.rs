use crate::game::{Game, GameState};

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

    fn num_players() -> usize {
        4
    }

    fn players_to_notify(&self) -> Vec<usize> {
        todo!()
    }

    fn update(&self) -> [u8; 32] {
        todo!()
    }

    fn apply_update(&mut self, update: [u8; 32]) {
        todo!()
    }

    fn console_move(&mut self, name: &String) {
        todo!()
    }

    fn network_move(&mut self, data: [u8; 30], received: usize, player: usize) {
        todo!()
    }

    fn make_move(&mut self, mv: Self::Move) {
        todo!()
    }

    fn get_gamestate(&self) -> GameState {
        todo!()
    }

    fn move_to_network(mv: Self::Move) -> [u8; 30] {
        todo!()
    }

    fn print_state(&self) {
        todo!()
    }
}