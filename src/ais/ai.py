from abc import ABC, abstractmethod


class AI(ABC):

    @abstractmethod
    def get_next_move(self, game_state):
        pass

    @abstractmethod
    def get_name(self):
        pass
