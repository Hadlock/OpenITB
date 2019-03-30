#/usr/bin/python3
'''this handles the models, namely the pieces and boardlocation'''
from .constants import EMPTY_SQUARE

class Model(object):
    '''create a chess board with pieces '''
    def __init__(self):
        '''create a chess board with pieces positioned for a new game
        row ordering is reversed from normal chess representations
        but corresponds to a top left screen coordinate

        self.board is a list of 8 character arrays
        '''

        # Generate a game map to be displayed
        self.tilemap = []
        selectedmap = '''G S S S S G G T
G G G G G G T T
M G G G G G D D
M T G G W G D M
T G G G G G G M
T G W G I T G M
G G G G W G G M
G T G G G I G T'''
        for line in selectedmap.splitlines():
            self.tilemap.append(line.split(" "))
        #print(self.tilemap)

        self.board = []
        for i in range(3):
            self.board.append([EMPTY_SQUARE]*8)
        start = ". . P P . P . ."
        self.board.append(start.split(" "))
        self.board.append([EMPTY_SQUARE]*8)
        firstlinee = ". . . . H . . ."
        self.board.append(firstlinee.split(" "))
        self.board.append([EMPTY_SQUARE]*8)
        secondlinee = ". . . . . H . ."
        self.board.append(secondlinee.split(" "))

        '''
        pawn_base = "P "*8
        white_pieces = "R N B Q K B N R"
        white_pawns = pawn_base.strip()
        black_pieces = white_pieces.lower()
        black_pawns = white_pawns.lower()
        self.board.append(black_pieces.split(" "))
        self.board.append(black_pawns.split(" "))
        for i in range(4):
            self.board.append([EMPTY_SQUARE]*8)
        self.board.append(white_pawns.split(" "))
        self.board.append(white_pieces.split(" "))
        '''

    def move(self, start, destination):
        ''' move a piece located at the start location to destination
        (each an instance of BoardLocation)
        Does not check whether the move is valid for the piece
        '''
        # error checking
        for check in [start, destination]:  # check coordinates are valid
            if check.i > 7 or check.j > 7 or check.i < 0 or check.j < 0:
                return
        if start.i == destination.i and start.j == destination.j: # don't move to same location
            return

        if self.board[start.i][start.j] == EMPTY_SQUARE:  #nothing to move
            return

        fboard = self.board[start.i][start.j]
        self.board[destination.i][destination.j] = fboard
        self.board[start.i][start.j] = EMPTY_SQUARE

class BoardLocation(object):
    ''' defines board location '''
    def __init__(self, i, j):
        self.i = i
        self.j = j
