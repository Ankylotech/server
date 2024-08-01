from abc import ABC, abstractmethod


class Game(ABC):

    @staticmethod
    @abstractmethod
    def game_identifier():
        pass

    @staticmethod
    def generic_identifier():
        return ['g', 'a', 'm', 'e']

    @staticmethod
    @abstractmethod
    def identifier():
        pass

    @abstractmethod
    def num_players(self):
        pass

    @abstractmethod
    def apply_update(self, update):
        pass

    @abstractmethod
    def console_move(self, name):
        pass

    @abstractmethod
    def make_move(self, mv):
        pass

    @abstractmethod
    def get_gamestate(self):
        pass

    @abstractmethod
    def move_to_network(self, mv):
        pass

    @abstractmethod
    def print_state(self):
        pass
