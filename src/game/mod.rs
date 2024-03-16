pub mod hearts;
pub mod tictactoe;

pub trait Game {
    type Move;
    
    fn game_identifier() -> [u8; 6];

    fn generic_identifier() -> [u8; 4] {
        ['g' as u8, 'a' as u8, 'm' as u8, 'e' as u8]
    }

    fn identifier() -> [u8;10] {
        let mut result = [0;10];
        let generic = Self::generic_identifier();
        let game = Self::game_identifier();
        for i in 0..10{
            if i < 4 {
                result[i] = generic[i];
            } else {
                result[i] = game[i-4];
            }
        }
        result
    }
}