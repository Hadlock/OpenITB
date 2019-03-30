#/usr/bin/python3
''' handles views'''
# pylint: disable=R0901
import os
import tkinter as tk
from tkinter import PhotoImage
from .constants import TILE_WIDTH, EMPTY_SQUARE,\
    ISOMETRIC_TILE_WIDTH, ISOMETRIC_TILE_HEIGHT, BOARD_WIDTH, \
    BOARD_HEIGHT, ISOMETRIC_BOARD_WIDTH, \
    DATA_DIR, ISOMETRIC_DATA_DIR, \
    ISOMETRIC_TILES, TILES, COLUMN_REFERENCE

class IsometricView(tk.Frame):
    ''' three quarter overhead view '''
    def __init__(self, parent=None):
        tk.Frame.__init__(self, parent)
        self.parent = parent
        self.preload_images()
        self.canvas_height = 7*ISOMETRIC_TILE_HEIGHT+self.board_y_offset
        self.canvas = tk.Canvas(self, width=ISOMETRIC_BOARD_WIDTH, height=self.canvas_height)
        self.canvas.pack()
        self.parent.title("OpenITB")
        self.pack()

    def preload_images(self):
        ''' preload images from disk '''
        self.images = {}
        for image_file_name in ISOMETRIC_TILES:
            f = os.path.join(ISOMETRIC_DATA_DIR, ISOMETRIC_TILES[image_file_name]) # pylint: disable=C0103
            if not os.path.exists(f):
                print("Error: Cannot find image file: %s at %s - aborting"%(ISOMETRIC_TILES[image_file_name], f)) # pylint: disable=C0301
                exit(-1)
            self.images[image_file_name] = PhotoImage(file=f)
        tallest = 0
        # pylint: disable=C0103
        for k, im in self.images.items(): # pylint: disable=W0612
            h = im.height()# pylint: disable=C0103
            if h > tallest:
                tallest = h

        self.board_y_offset = tallest

    def clear_canvas(self):
        ''' delete everything from the canvas'''
        items = self.canvas.find_all()
        for i in items:
            self.canvas.delete(i)

    def draw_empty_board(self, debug_board=1):
        ''' draw an empty board on the canvas
        if debug_board is set  show the coordinates of each of the tile corners'''
        for j in range(8): # rows, or y coordinates
            for i in range(8): # columns, or x coordinates
                x, y = self.get_tile_sw(i, j) # pylint: disable=C0103
                drawing_order = j*8 + i
                tile_black = (j+i)%2
                if tile_black == 0:
                    tile = self.images['black_tile']
                else:
                    tile = self.images['white_tile']
                self.canvas.create_image(x, y, anchor=tk.SW, image=tile)

                # debug_board = True
                if debug_board:  # implicitly this means if debug_board == True.
                    ''' If we are drawing a debug board, draw an arrow showing top left
                    and its coordinates. ''' # pylint: disable=W0105
                    current_tile = drawing_order +1 # (start from 1)
                    text_pos = (x+ISOMETRIC_TILE_WIDTH/2, y-ISOMETRIC_TILE_HEIGHT/2)
                    line_end = (x+ISOMETRIC_TILE_WIDTH/4, y -ISOMETRIC_TILE_HEIGHT/4)
                    self.canvas.create_line((x, y), line_end, arrow=tk.FIRST)
                    text_content = "(%s: %s,%s)"%(current_tile, x, y)
                    self.canvas.create_text(text_pos, text=text_content)

    def get_tile_sw(self, i, j):
        ''' given a row and column location for a piece return the x,y coordinates
        of the bottom left hand corner of
        the tile '''

        y_start = (j*ISOMETRIC_TILE_HEIGHT/2)+self.board_y_offset
        x_start = (7-j)*ISOMETRIC_TILE_WIDTH/2
        x = x_start+(i*ISOMETRIC_TILE_WIDTH/2) # pylint: disable=C0103
        y = y_start +(i*ISOMETRIC_TILE_HEIGHT/2) # pylint: disable=C0103

        return (x, y)

    def draw_map_board(self, tilemap):
        '''draws map tiles over the board'''
        for j, row in enumerate(tilemap):
            for i, piece in enumerate(reversed(row)):
                tile = self.images[piece]
                x, y = self.get_tile_sw(j, i) # pylint: disable=C0103
                self.canvas.create_image(x, y, anchor=tk.SW, image=tile)

    def draw_pieces(self, board):
        ''' draws the pieces on the board '''
        for j, row in enumerate(board):  # this is the rows = y axis
            # using enumerate we get an integer index
            # for each row which we can use to calculate y
            # because rows run down the screen, they correspond to the y axis
            # and the columns correspond to the x axis
            # isometric pieces need to be drawn by reference to a bottom corner of the tile,
            #  We are using SW (ie bottom left).
            for i, piece in enumerate(reversed(row)): # columns = x axis
                if piece == EMPTY_SQUARE:
                    continue  # skip empty tilesdraw_pieces
                tile = self.images[piece]
                x, y = self.get_tile_sw(j, i) # pylint: disable=C0103
                self.canvas.create_image(x, y, anchor=tk.SW, image=tile)

    def display(self, board, tilemap, debug_board=False):
        ''' draw an empty board then draw each of the
        pieces in the board over the top'''

        self.clear_canvas()
        self.draw_empty_board(debug_board=debug_board)
        self.draw_map_board(tilemap)
        if not debug_board:
            self.draw_pieces(board)

        print("%s: %s"%(" ", COLUMN_REFERENCE))
        print("-"*50)
        for i, row in enumerate(board):
            row_marker = 8-i
            print("%s: %s"%(row_marker, row))

    def display_debug_board(self):
        ''' draws debug board '''
        self.clear_canvas()
        self.draw_empty_board()

class View(tk.Frame):
    ''' view '''
    def __init__(self, parent=None):
        tk.Frame.__init__(self, parent)
        self.canvas = tk.Canvas(self, width=BOARD_WIDTH, height=BOARD_HEIGHT)
        self.canvas.pack()
        self.images = {}
        for image_file_name in TILES:
            f = os.path.join(DATA_DIR, TILES[image_file_name]) # pylint: disable=C0103
            if not os.path.exists(f):
                print("Error: Cannot find image file: %s at %s - aborting"%(TILES[image_file_name], f)) # pylint: disable=C0301
                exit(-1)
            self.images[image_file_name] = PhotoImage(file=f)
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

    def draw_row(self, y, first_tile_white=True, debug_board=False): # pylint: disable=C0103
        ''' draw a single row of alternating black and white tiles,
        the colour of the first tile is determined by first_tile_white
        if debug_board is set  show the coordinates of each of the tile corners
        '''

        if first_tile_white:
            remainder = 1
        else:
            remainder = 0
        for i in range(8):
            x = i*TILE_WIDTH # pylint: disable=C0103
            if i%2 == remainder:
                # i %2 is the remainder after dividing i by 2
                # so i%2 will always be either 0 (no remainder- even numbers) or
                # 1 (remainder 1 - odd numbers)
                # this tests whether the number i is even or odd
                tile = self.images['black_tile']
            else:
                tile = self.images['white_tile']
            self.canvas.create_image(x, y, anchor=tk.NW, image=tile)
            # NW is a constant in the Tkinter module.  It stands for "north west"
            # that is, the top left corner of the picture is to be located at x,y
            # if we used another anchor, the grid would not line up properly with
            # the canvas size
            if debug_board:  # implicitly this means if debug_board == True.
                ''' If we are drawing a debug board, draw an arrow showing top left
                and its coordinates. ''' # pylint: disable=W0105
                text_pos = (x+TILE_WIDTH/2, y+TILE_WIDTH/2)
                line_end = (x+TILE_WIDTH/4, y +TILE_WIDTH/4)
                self.canvas.create_line((x, y), line_end, arrow=tk.FIRST)
                text_content = "(%s,%s)"%(x, y)
                self.canvas.create_text(text_pos, text=text_content)


    def draw_empty_board(self, debug_board=False):
        ''' draw an empty board on the canvas
        if debug_board is set  show the coordinates of each of the tile corners'''
        y = 0 # pylint: disable=C0103
        for i in range(8): # draw 8 rows
            y = i*TILE_WIDTH # pylint: disable=C0103
            # each time, advance the y value at which the row is drawn
            # by the length of the tile
            first_tile_white = not i%2
            self.draw_row(y, first_tile_white, debug_board)

    def draw_pieces(self, board):
        ''' draws pieces on board '''
        for i, row in enumerate(board):
            # using enumerate we get an integer index
            # for each row which we can use to calculate y
            # because rows run down the screen, they correspond to the y axis
            # and the columns correspond to the x axis
            for j, piece in enumerate(row):
                if piece == EMPTY_SQUARE:
                    continue  # skip empty tiles
                tile = self.images[piece] # map the piece to the screen
                x = j*TILE_WIDTH # pylint: disable=C0103
                y = i*TILE_WIDTH # pylint: disable=C0103
                self.canvas.create_image(x, y, anchor=tk.NW, image=tile)

    def display(self, board, debug_board=False):
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
        ''' displays debug board '''
        self.clear_canvas()
        self.draw_empty_board()
