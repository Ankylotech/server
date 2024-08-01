use crate::ais::AI;
use crate::game::tictactoe::TicTacToe;
use crate::game::{Game, GameState};

pub struct TicTacToeAI;

impl TicTacToeAI {
    fn min_max_search(state: &TicTacToe, depth: usize) -> i32 {
        if depth == 0 || !(state.get_gamestate() == GameState::ONGOING) {
            return match state.get_gamestate() {
                GameState::ONGOING => 0,
                GameState::DRAW => 0,
                GameState::WINNER(i) => -1,
            };
        }
        -state
            .legal_turns()
            .iter()
            .map(|x| {
                let mut s = state.clone();
                s.make_move(*x);
                Self::min_max_search(&mut s, depth - 1)
            })
            .min()
            .unwrap()
    }
}

impl AI<TicTacToe> for TicTacToeAI {
    fn get_next_move(self: &Self, state: &TicTacToe) -> <TicTacToe as Game>::Move {
        let mut min = 1;
        let mut result = 0;
        for m in state.legal_turns() {
            let mut s = state.clone();
            s.make_move(m);
            let v = Self::min_max_search(&s, 9);
            if v < min {
                min = v;
                result = m;
            }
        }
        result
    }

    fn get_name(self: &Self) -> String {
        "testai".to_string()
    }
}
