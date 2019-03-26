#/usr/bin/python3

column_reference = "a b c d e f g h".split(" ")
EMPTY_SQUARE = " "

class Model(object):
    def __init__(self):
        '''create a chess board with pieces positioned for a new game
        row ordering is reversed from normal chess representations
        but corresponds to a top left screen coordinate
        '''

        self.board = []
        pawn_base = "P "*8
        white_pieces =  "R N B Q K B N R"
        white_pawns = pawn_base.strip()
        black_pieces = white_pieces.lower()
        black_pawns = white_pawns.lower()
        self.board.append(black_pieces.split(" "))
        self.board.append(black_pawns.split(" "))
        for i in range(4):
            self.board.append([EMPTY_SQUARE]*8)
        self.board.append(white_pawns.split(" "))
        self.board.append(white_pieces.split(" "))


    def move(self, start,  destination):
        ''' move a piece located at the start location to destination
        (each an instance of BoardLocation)
        Does not check whether the move is valid for the piece
        '''
        # error checking
        for c in [start, destination]:  # check coordinates are valid
            if c.i > 7 or c.j > 7 or c.i <0 or c.j <0:
                return
        if start.i == destination.i and start.j == destination.j: # don't move to same location
            return

        if self.board[start.i][start.j] == EMPTY_SQUARE:  #nothing to move
            return

        f = self.board[start.i][start.j]
        self.board[destination.i][destination.j] = f
        self.board[start.i][start.j] = EMPTY_SQUARE

class BoardLocation(object):
    def __init__(self, i, j):
        self.i = i
        self.j = j
