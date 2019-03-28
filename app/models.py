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

        self.board = []
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
