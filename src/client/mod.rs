trait AI<T:Game> {
    pub fn get_next_move(self: Self, state: Game) -> Move;
}