use crate::game::{Game, GameState};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TicTacToe {
    board: [[i32; 3]; 3],
    turn: u8,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[0; 3]; 3],
            turn: 0,
        }
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

    pub fn legal_turns(&self) -> Vec<<TicTacToe as Game>::Move> {
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
}

impl Game for TicTacToe {
    type Move = u8;

    fn game_identifier() -> [u8; 6] {
        [
            't' as u8, 'i' as u8, 't' as u8, 'a' as u8, 't' as u8, 'o' as u8,
        ]
    }

    fn num_players() -> usize {
        2
    }

    fn players_to_notify(&self) -> Vec<usize> {
        if (self.turn % 2 == 0) {
            vec![0]
        } else {
            vec![1]
        }
    }

    fn update(&self) -> [u8; 32] {
        let mut result = [0; 32];
        for i in 0..3 {
            for j in 0..3 {
                result[i * 3 + j] = (self.board[i][j] + 1) as u8;
            }
        }
        result[9] = self.turn;
        result
    }

    fn apply_update(&mut self, update: [u8; 32]) {
        self.turn = update[9];
        for i in 0..3 {
            for j in 0..3 {
                self.board[i][j] = update[i * 3 + j] as i32 - 1;
            }
        }
    }

    fn console_move(&mut self, name: &String) {
        self.print_game();
        println!("Legal turns are:");
        for t in self.legal_turns() {
            print!("{}, ", t + 1);
        }
        println!();
        println!("What do you want to play {}?", name);
        let mut input = String::new();
        while !input
            .parse::<u8>()
            .is_ok_and(|i| i > 0 && i < 10 && self.legal_turns().contains(&(i - 1)))
        {
            println!("Input a legal number between 1 and 9");
            input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error while reading input");
            input = input.trim().to_string();
        }
        self.make_move((input.parse::<u8>().unwrap() - 1));
    }
    fn network_move(&mut self, data: [u8; 30], received: usize, player: usize) {
        println!("received {:?}", data);
        if data[1] == self.turn && player as u8 == self.turn % 2 {
            self.make_move(data[0]);
        }
    }

    fn make_move(&mut self, turn: <TicTacToe as Game>::Move) {
        let x = turn / 3;
        let y = turn % 3;
        self.board[x as usize][y as usize] = if self.turn % 2 == 0 { 1 } else { -1 };
        self.turn += 1;
    }

    fn get_gamestate(&self) -> GameState {
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
                    return GameState::WINNER((-totals[i][j].signum() + 1) as usize / 2);
                }
            }
        }
        if (moves == 9) {
            GameState::DRAW
        } else {
            GameState::ONGOING
        }
    }

    fn move_to_network(&self, mv: Self::Move) -> [u8; 30] {
        let mut l = [0; 30];
        l[0] = mv;
        l[1] = self.turn;
        l
    }

    fn print_state(&self) {
        self.print_game();
    }

    fn default_move(&mut self, player: usize) {
        self.make_move(self.legal_turns()[0]);
    }
    /*
    fn get_turn(&self) -> bool {
        self.turn
    }

    fn input_size() -> usize {
        9
    }
    */
}
