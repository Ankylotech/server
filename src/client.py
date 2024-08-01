import socket

from ais.test import TicTacToeAI
from game.tictactoe import TicTacToe

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(('', 34255))
print("bound to localhost")
data, addr = sock.recvfrom(10)
print("connected")
if data == TicTacToe.identifier():
    ai = TicTacToeAI()
    game = TicTacToe()
else:
    exit()

sock.sendto(ai.get_name(), addr)

while game.get_gamestate() == -2:
    data, addr2 = sock.recvfrom(32)
    if addr != addr2:
        continue
    if not game.apply_update(data):
        continue
    print("got update")
    mv = ai.get_next_move(game)
    print("ai decided on " + str(mv))
    msg = game.move_to_network(mv)
    print("sending " + str(msg))
    sock.sendto(msg, addr)

sock.close()