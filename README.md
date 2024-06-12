# A Competition to create the best AIs at different games

### Goals

The goal of this program is to allow the hosting of a Programming-competition with different games and multiple AIs. 
The AIs should be allowed to run in as many different Programming-languages as possible, among them has to be Python to 
allow people with minimum experience in programming as possible to participate.

To achieve this the tournament is hosted on a server and communicates with the clients using UDP messages. 

It would be nice if the competition allows for the participants to compete in multiple different games at once and if
the architecture allows for local and fast testing of your own AIs against themselves in many matches or others to 
comprehensively test them.


### Current Architecture:

Currently the server and clients have to follow the following steps to play:

+ The server sends a message containing the game identifier to the Broadcast adress 255.255.255.255:34255. 
+ The client responds with a message containing a (hopefully) unique name of at most 30 Bytes directly to the server.
+ Each turn the server sends a message to client containing 32 Bytes with an update to the current game state
+ The client then has a specified amount of time to respond with a move in 30 Bytes.

#### Game Requirements:

Each game has to support the Game Interface as specified in src/game/mod.rs. The following things are important for
both server and client:

- A function that returns a unique identifier
- A function that returns the current game status as ongoing, a winner or a draw

The server requires the following functions:

- A function that returns the number of players
- A function that returns the players that have to be updated with the current status
- A function that returns 32 Bytes that form a update of the game state 
- A function that applies a move with the Data coming from the client
- A function that reads a move from the console
- A function that makes a move directly for local play
- A function that prints the current game state to the console
- A function that makes a default move when no response is received in the appropriate time

The client needs the following functionality:

- A function that applies a update coming from the server
- A function that transforms a game move to a 30 Byte message that can be sent to the server

Additional functionality might be useful for the client to choose a good move;

### Current games:

#### TicTacToe

- Identifier: 'gametitato'
- Num_players: 2
- players to notify: 0/1 depending on the turn
- update of the game state: Bytes 0 - 8: Board -- 0 P1, 1: noone, 2:P2; Byte 9: Turn-number
- Network move: Byte 0: Position on the board, Byte 1: Turn number in case message arrives late than expected