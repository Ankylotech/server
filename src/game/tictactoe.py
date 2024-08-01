from game.game import Game

sign = lambda x: x and (1, -1)[x < 0]


class TicTacToe(Game):
    @staticmethod
    def identifier():
        return b"gametitato"

    def __init__(self):
        self.board = [[0, 0, 0] for _ in range(3)]
        self.turn = 0

    def legal_moves(self):
        legal = []
        for i in range(9):
            if self.board[i // 3][i % 3] == 0:
                legal.append(i)

        return legal

    def game_identifier(self):
        return ['t', 'i', 't', 'a', 't', 'o']

    def num_players(self):
        return 2

    def apply_update(self, update):
        prev = self.turn
        self.turn = update[9]
        for i in range(3):
            for j in range(3):
                self.board[i][j] = update[i * 3 + j] - 1
        return prev != self.turn

    def console_move(self, name):
        while True:
            self.print_state()
            move = input(name + " please input the next move to make: \n")
            if int(move) in self.legal_moves():
                self.make_move(int(move))
                return

    def make_move(self, turn):
        x = turn // 3
        y = turn % 3
        self.board[x][y] = 1 if self.turn % 2 == 0 else -1
        self.turn += 1

    # returns -2 in an ongoing game, -1 for draw and the index of the winning player otherwise
    def get_gamestate(self):
        totals = [[0, 0, 0] for _ in range(3)]
        for i in range(3):
            for j in range(3):
                totals[i][0] += self.board[i][j]
                totals[j][1] += self.board[i][j]
                if i == j:
                    totals[0][2] += self.board[i][j]

                if 2 - i == j:
                    totals[1][2] += self.board[i][j]

        for i in range(3):
            for j in range(3):
                if abs(totals[i][j]) == 3:
                    return (sign(-totals[i][j]) + 1) // 2

        if self.turn == 9:
            return -1
        else:
            return -2

    def move_to_network(self, mv):
        data = [0 for _ in range(30)]
        data[0] = mv
        data[1] = self.turn
        return bytearray(data)

    def print_state(self):
        for i in range(3):
            for j in range(3):
                if self.board[i][j] == 1:
                    c = 'X'
                elif self.board[i][j] == -1:
                    c = 'O'
                else:
                    c = ' '
                print("{}", c)
                if j < 2:
                    print("|")
            print("\n")
            if i < 2:
                print("-+-+-\n")
