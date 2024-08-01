from copy import deepcopy

from ais.ai import AI


class TicTacToeAI(AI):

    @staticmethod
    def min_max_search(game, depth):
        if depth == 0 or not (game.get_gamestate() == -2):
            if game.get_gamestate() == -1:
                return 0
            else:
                return -1

        result = 2
        for t in game.legal_moves():
            g = deepcopy(game)
            g.make_move(t)
            v = TicTacToeAI.min_max_search(g, depth)
            if v < result:
                result = v
        return -1 * result

    @staticmethod
    def get_next_move(game):
        m = 2
        result = 0
        for t in game.legal_moves():
            g = deepcopy(game)
            g.make_move(t)
            v = TicTacToeAI.min_max_search(g, 9)
            if v < m:
                m = v
                result = t
        return result

    @staticmethod
    def get_name():
        return b"pytestai"
