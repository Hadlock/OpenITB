#/usr/bin/python3
''' handles views'''
import os
import tkinter as tk
from tkinter import PhotoImage

TILE_WIDTH = 60
EMPTY_SQUARE = " "

ISOMETRIC_TILE_WIDTH = 120
ISOMETRIC_TILE_HEIGHT = 60
# to display these tiles many locations in the code rely on integer division by 2,
# so this width and height should both be even numbers (otherwise rounding errors will accumulate)

BOARD_WIDTH = 8*TILE_WIDTH
BOARD_HEIGHT = BOARD_WIDTH

ISOMETRIC_BOARD_WIDTH = 8 * ISOMETRIC_TILE_WIDTH
ISOMETRIC_BOARD_HEIGHT= 8*ISOMETRIC_TILE_HEIGHT

DATA_DIR = "chess_data"
ISOMETRIC_DATA_DIR = "chess_data"

DATA_DIR = "chess_data"
ISOMETRIC_DATA_DIR = "chess_data"

ISOMETRIC_TILES = {"black_tile":"chess_isometric_black_tile.gif",
    "B":"chess_isometric_white_bishop1.gif",
    "b":"chess_isometric_black_bishop1.gif",
    "k":"chess_isometric_black_king1.gif",
    "K":"chess_isometric_white_king1.gif",
    "n":"chess_isometric_black_knight1.gif",
    "N":"chess_isometric_white_knight1.gif",
    "p":"chess_isometric_black_pawn1.gif",
    "P":"chess_isometric_white_pawn1.gif",
    "q":"chess_isometric_black_queen1.gif",
    "Q":"chess_isometric_white_queen1.gif",
    "r":"chess_isometric_black_rook1.gif",
    "R":"chess_isometric_white_rook1.gif",
    "white_tile":"chess_isometric_white_tile.gif"
    }

TILES = {"black_tile":"black_tile.gif",
    # leaving this here for possible future debug
    "B":"chess_b451.gif",
    "b":"chess_b45.gif",
    "k":"chess_k45.gif",
    "K":"chess_k451.gif",
    "n":"chess_n45.gif",
    "N":"chess_n451.gif",
    "p":"chess_p45.gif",
    "P":"chess_p451.gif",
    "q":"chess_q45.gif",
    "Q":"chess_q451.gif",
    "r":"chess_r45.gif",
    "R":"chess_r451.gif",
    "white_tile":"white_tile.gif"
    }

class View(tk.Frame):
    ''' view '''
    def __init__(self,  parent = None):
        tk.Frame.__init__(self, parent)
        self.canvas = tk.Canvas(self, width=BOARD_WIDTH, height=BOARD_HEIGHT)
        self.canvas.pack()
        self.images = {}
        for image_file_name in TILES:
            f = os.path.join(DATA_DIR, TILES[image_file_name])
            if not os.path.exists(f):
                print("Error: Cannot find image file: %s at %s - aborting"%(TILES[image_file_name], f))
                exit(-1)
            self.images[image_file_name]= PhotoImage(file=f)
            '''This opens each of the image files, converts the data into a form that Tkinter
            can use, then stores that converted form in the attribute self.images
            self.images is a dictionary, keyed by the letters we used in our model to
            represent the pieces - ie PRNBKQ for white and prnbkq for black
            eg self.images['N'] is a PhotoImage of a white knight
            this means we can directly translate a board entry from the model into a picture
            '''
        self.pack()


    def clear_canvas(self):
        ''' delete everything from the canvas'''
        items = self.canvas.find_all()
        for i in items:
            self.canvas.delete(i)

    def draw_row(self, y,  first_tile_white=True,  debug_board = False):
        ''' draw a single row of alternating black and white tiles,
        the colour of the first tile is determined by first_tile_white
        if debug_board is set  show the coordinates of each of the tile corners
        '''

        if first_tile_white:
            remainder = 1
        else:
            remainder = 0
        for i in range(8):
            x = i*TILE_WIDTH
            if i%2 == remainder:
                # i %2 is the remainder after dividing i by 2
                # so i%2 will always be either 0 (no remainder- even numbers) or
                # 1 (remainder 1 - odd numbers)
                # this tests whether the number i is even or odd
                tile = self.images['black_tile']
            else:
                tile = self.images['white_tile']
            self.canvas.create_image(x, y, anchor = tk.NW,  image=tile)
            # NW is a constant in the Tkinter module.  It stands for "north west"
            # that is, the top left corner of the picture is to be located at x,y
            # if we used another anchor, the grid would not line up properly with
            # the canvas size
            if debug_board:  # implicitly this means if debug_board == True.
                ''' If we are drawing a debug board, draw an arrow showing top left
                and its coordinates. '''
                text_pos =  (x+TILE_WIDTH/2, y+TILE_WIDTH/2)
                line_end = (x+TILE_WIDTH/4,  y +TILE_WIDTH/4)
                self.canvas.create_line((x, y), line_end,  arrow = tk.FIRST)
                text_content = "(%s,%s)"%(x, y)
                self.canvas.create_text(text_pos, text=text_content)


    def draw_empty_board(self,  debug_board = False):
        ''' draw an empty board on the canvas
        if debug_board is set  show the coordinates of each of the tile corners'''
        y = 0
        for i in range(8): # draw 8 rows
            y = i*TILE_WIDTH
            # each time, advance the y value at which the row is drawn
            # by the length of the tile
            first_tile_white =  not (i%2)
            self.draw_row(y, first_tile_white,  debug_board )

    def draw_pieces(self, board):
        for i, row in enumerate(board):
            # using enumerate we get an integer index
            # for each row which we can use to calculate y
            # because rows run down the screen, they correspond to the y axis
            # and the columns correspond to the x axis
            for j,  piece in enumerate(row):
                if piece == EMPTY_SQUARE:
                    continue  # skip empty tiles
                tile = self.images[piece]
                x = j*TILE_WIDTH
                y = i*TILE_WIDTH
                self.canvas.create_image(x, y, anchor=tk.NW,  image = tile)

    def display(self, board,  debug_board= False):
        ''' draw an empty board then draw each of the
        pieces in the board over the top'''

        self.clear_canvas()
        self.draw_empty_board(debug_board=debug_board)
        if not debug_board:
            self.draw_pieces(board)

        # first draw the empty board
        # then draw the pieces
        # if the order was reversed, the board would be drawn over the pieces
        # so we couldn't see them

    def display_debug_board(self):
        self.clear_canvas()
        self.draw_empty_board()

class Isometric_View(tk.Frame):
    ''' three quarter overhead view '''
    def __init__(self,  parent = None):
        tk.Frame.__init__(self, parent)
        self.parent = parent
        self.preload_images()
        self.canvas_height = 7*ISOMETRIC_TILE_HEIGHT+self.board_y_offset
        self.canvas = tk.Canvas(self, width=ISOMETRIC_BOARD_WIDTH, height=self.canvas_height)
        self.canvas.pack()
        self.parent.title("Python4Kids")
        self.pack()

    def preload_images(self):
        self.images = {}
        for image_file_name in ISOMETRIC_TILES:
            f = os.path.join(ISOMETRIC_DATA_DIR, ISOMETRIC_TILES[image_file_name])
            if not os.path.exists(f):
                print("Error: Cannot find image file: %s at %s - aborting"%(ISOMETRIC_TILES[image_file_name], f))
                exit(-1)
            self.images[image_file_name]= PhotoImage(file=f)
        tallest = 0
        for k, im in self.images.items():
            h = im.height()
            if h > tallest:
                tallest = h

        self.board_y_offset = tallest

    def clear_canvas(self):
        ''' delete everything from the canvas'''
        items = self.canvas.find_all()
        for i in items:
            self.canvas.delete(i)

    def draw_empty_board(self,  debug_board = False):
        ''' draw an empty board on the canvas
        if debug_board is set  show the coordinates of each of the tile corners'''

        for j in range(8): # rows, or y coordinates
            for i in range(8): # columns, or x coordinates
                x, y = self.get_tile_sw(i, j)
                drawing_order = j*8 + i
                tile_white = (j+i)%2
                if tile_white == 0:
                    tile = self.images['white_tile']
                else:
                    tile = self.images['black_tile']
                self.canvas.create_image(x, y, anchor = tk.SW,  image=tile)

                if debug_board:  # implicitly this means if debug_board == True.
                    ''' If we are drawing a debug board, draw an arrow showing top left
                    and its coordinates. '''
                    current_tile = drawing_order +1 # (start from 1)

                    text_pos =  (x+ISOMETRIC_TILE_WIDTH/2, y-ISOMETRIC_TILE_HEIGHT/2)
                    line_end = (x+ISOMETRIC_TILE_WIDTH/4,  y -ISOMETRIC_TILE_HEIGHT/4)
                    self.canvas.create_line((x, y), line_end,  arrow = tk.FIRST)
                    text_content = "(%s: %s,%s)"%(current_tile, x, y)
                    self.canvas.create_text(text_pos, text=text_content)

    def get_tile_sw(self, i,  j):
        ''' given a row and column location for a piece return the x,y coordinates of the bottom left hand corner of
        the tile '''

        y_start = (j*ISOMETRIC_TILE_HEIGHT/2)+self.board_y_offset
        x_start = (7-j)*ISOMETRIC_TILE_WIDTH/2
        x = x_start+(i*ISOMETRIC_TILE_WIDTH/2)
        y = y_start +(i*ISOMETRIC_TILE_HEIGHT/2)

        return (x, y)

    def draw_pieces(self, board):
        for j, row in enumerate(board):  # this is the rows = y axis
            # using enumerate we get an integer index
            # for each row which we can use to calculate y
            # because rows run down the screen, they correspond to the y axis
            # and the columns correspond to the x axis
            # isometric pieces need to be drawn by reference to a bottom corner of the tile,  We are using
            # SW  (ie bottom left).

            for i,  piece in enumerate(row): # columns = x axis
                if piece == EMPTY_SQUARE:
                    continue  # skip empty tiles
                tile = self.images[piece]
                x, y = self.get_tile_sw(i, j)
                self.canvas.create_image(x, y, anchor=tk.SW,  image = tile)

    def display(self, board,  debug_board= False):
        ''' draw an empty board then draw each of the
        pieces in the board over the top'''

        self.clear_canvas()
        self.draw_empty_board(debug_board=debug_board)
        if not debug_board:
            self.draw_pieces(board)

        # first draw the empty board
        # then draw the pieces
        # if the order was reversed, the board would be drawn over the pieces
        # so we couldn't see them

    def display_debug_board(self):
        self.clear_canvas()
        self.draw_empty_board()
