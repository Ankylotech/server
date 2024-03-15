use crate::game::Game;

use super::Eval;
use rand::Rng;

#[derive(Clone, PartialEq, Eq,Debug)]
pub struct TicTacToe {
    board: [[i32; 3]; 3],
    turn: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[0; 3]; 3],
            turn: true,
        }
    }
}

impl Game for TicTacToe {
    type Move = u8;

    fn legal_turns(&self) -> Vec<Self::Move> {
        let mut turns = Vec::new();

        for i in 0..3 {
            for j in 0..3 {
                if self.board[i][j] == 0 {
                    turns.push((i * 3 + j) as u8);
                }
            }
        }
        turns
    }

    fn take_turn(&mut self, turn: &Self::Move) {
        let x = turn / 3;
        let y = turn % 3;
        self.board[x as usize][y as usize] = if self.turn { 1 } else { -1 };
        self.turn = !self.turn;
    }

    fn game_over(&self) -> (bool, i32) {
        let mut totals = [[0; 3]; 3];
        let mut moves = 0;
        for i in 0..3 {
            for j in 0..3 {
                totals[i][0] += self.board[i][j];
                totals[j][1] += self.board[i][j];
                if i == j {
                    totals[0][2] += self.board[i][j];
                }
                if 2 - i == j {
                    totals[1][2] += self.board[i][j];
                }

                moves += self.board[i][j].abs();
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                if totals[i][j].abs() == 3 {
                    return (true, totals[i][j].signum());
                }
            }
        }
        (moves == 9, 0)
    }

    fn print_game(&self) {
        for i in 0..3 {
            for j in 0..3 {
                let c = match self.board[i][j] {
                    1 => 'X',
                    -1 => 'O',
                    _ => ' ',
                };
                print!("{}", c);
                if j < 2 {
                    print!("|");
                }
            }
            println!();
            if i < 2 {
                println!("-+-+-");
            }
        }
    }

    fn human_turn(&mut self) {
        self.print_game();
        println!("Legal turns are:");
        for t in self.legal_turns() {
            print!("{}, ", t + 1);
        }
        println!();
        println!("What do you want to play?");
        let mut input = String::new();
        while !input
            .parse::<u8>()
            .is_ok_and(|i| *i > 0 && *i < 10 && self.legal_turns().contains(&(i - 1)))
        {
            println!("Input a legal number between 1 and 9");
            input = String::new();
            std::io::stdin().read_line(&mut input);
            input = input.trim().to_string();
        }
        self.take_turn(&(input.parse::<u8>().unwrap() - 1));
    }

    fn get_turn(&self) -> bool {
        self.turn
    }

    fn input_size() -> usize {
        9
    }
}