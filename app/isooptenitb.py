#/bin/python3
'''
Representing a chess set in Python
Part 3 (Isometric tiles)
Brendan Scott
4 May 2013
 
Dark square on a1
Requires there to be a directory called
chess_data in the current directory, and for that
data directory to have a copy of all the images
 
'''
 
import tkinter as tk
from tkinter import PhotoImage
import os.path
import os
#from . import openitb as oitb
#from .openitb import Model,  BoardLocation,  View,  TILES
# Use the Model class from the previous tutorial.
# rename mvc2 to whatever name you gave the script from that tutorial
# if you can't find the previous tutorial just uncomment the definition below.
 
column_reference = "a b c d e f g h".split(" ")
EMPTY_SQUARE = " "
 
TILE_WIDTH = 60
'''We have used a tile width of 60 because the images we are used are 60x60 pixels
The original svg files were obtained from
http://commons.wikimedia.org/wiki/Category:SVG_chess_pieces/Standard_transparent
after downloading they were batch converted to png, then gif files.  Bash one liners
(Unix) to do this:
for i in $(ls *.svg); do inkscape -e ${i%.svg}.png -w 60 -h 60 $i ; done
for i in $(ls *.png); do convert $i  ${i%.png}.gif ; done
white and black tiles were created in inkscape
 
Isometric tiles were created in inkscape
Isometric pieces were created with povray using ChessSets Version 1.2
by James Garner ( jkgarner@charter.net )
then post processed in GIMP.
 
'''
 
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

class View(tk.Frame):
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
 
class Controller(object):
    def __init__(self,  parent = None,  model = None):
        if model is None:
            self.m = Model()
        else:
            self.m = model
        self.v = Isometric_View(parent)
        self.v2 = View(parent)
 
        ''' we have created both a model and a view within the controller
        the controller doesn't inherit from either model or view
        '''
        self.v.canvas.bind("<Button-1>",  self.handle_click_isometric)
        # this binds the handle_click method to the view's canvas for left button down
        self.v2.canvas.bind("<Button-1>",  self.handle_click)
        self.clickList = []
        # I have kept clickList here, and not in the model, because it is a record of what is happening
        # in the view (ie click events) rather than something that the model deals with (eg moves).
 
    def run(self,  debug_mode = False):
        self.update_display(debug_board=debug_mode)
        tk.mainloop()
 
    def handle_click(self,  event):
        ''' Handle a click received.  The x,y location of the click on the canvas is at
        (event.x, event.y)
        First, we need to translate the event coordinates (ie the x,y of where the click occurred)
        into a position on the chess board
        add this to a list of clicked positions
        every first click is treated as a "from" and every second click as a"to"
        so, whenever there are an even number of clicks, use the most recent to two to perform a move
        then update the display
        '''
        j = event.x//TILE_WIDTH
        #  the / operator is called integer division
        # it returns the number of times TILE_WIDTH goes into event.x ignoring any remainder
        # eg: 2/2 = 1, 3/2 = 1, 11/5 = 2 and so on
        # so, it should return a number between 0 (if x < TILE_WIDTH) though to 7
        i = event.y//TILE_WIDTH
 
        self.clickList.append(BoardLocation(i, j))
        # just maintain a list of all of the moves
        # this list shouldn't be used to replay a series of moves because that is something
        # which should be stored in the model - but it wouldn't be much trouble to
        # keep a record of moves in the model.
        if len(self.clickList)%2 ==0:
            # move complete, execute the move
            self.m.move(self.clickList[-2], self.clickList[-1])
            # use the second last entry in the clickList and the last entry in the clickList
            self.update_display()
 
    def handle_click_isometric(self,  event):
        ''' Handle a click received.  The x,y location of the click on the canvas is at
        (event.x, event.y)
        First, we need to translate the event coordinates (ie the x,y of where the click occurred)
        into a position on the chess board
        add this to a list of clicked positions
        every first click is treated as a "from" and every second click as a"to"
        so, whenever there are an even number of clicks, use the most recent to two to perform a move
        then update the display
        '''
        i, j = self.xy_to_ij(event.x,  event.y)
        self.clickList.append(BoardLocation(7-i, j))  # 7-i because the Model stores the board in reverse row order.
        # just maintain a list of all of the moves
        # this list shouldn't be used to replay a series of moves because that is something
        # which should be stored in the model - but it wouldn't be much trouble to
        # keep a record of moves in the model.
        if len(self.clickList)%2 ==0:
            # move complete, execute the move
            self.m.move(self.clickList[-2], self.clickList[-1])
            # use the second last entry in the clickList and the last entry in the clickList
            self.update_display()
 
    def xy_to_ij(self, x, y):
        ''' given x,y coordinates on the screen, convert to a location on
        the virtual board.
        Involves non trivial mathematics
        The tiles have, in effect, two edges. One leading down to the right,
        and one leading up to the right. These define a (non-orthogonal) basis
        (look it up) for describing the screen.
        The first vector V1 is (60,-30), the second V2= (60,30), and the
        coordinates were given XY=(x,y)
        so we want to find two numbers a and b such that:
        aV1+bV2 = XY
        Where a represents the column and b represents the row
        or, in other words:
        a(60,-30)+b(60,30) = (x,y)
        60a +60b = x
        -30a +30b = y
        so
        b = (y+30a)/30.0
        and
        60a+60*(y+30a)/30 = x
        => 60a +2y+60a = x
        => 120a = x-2y
        a = (x-2y)/120
 
        HOWEVER, this is calculated by reference to the a1 corner of the board
        AND assumes that y increases going upwards not downwards.
 
        This corner is located at 8* ISOMETRIC_TILE_HEIGHT/2  from the bottom of the canvas
        (count them)
        so first translate the x,y coordinates we have received
        x stays the same
        '''
 
        y = self.v.canvas_height-y # invert it
        y = y - 4*ISOMETRIC_TILE_HEIGHT # Get y relative to the height of the corner
 
        a = (x-2*y)/120.0
        b = (y+30*a)/30.0
        # if either of these is <0 this means that the click is off the board (to the left or below)
        # if the number is greater than -1, but less than 0, int() will round it up to 0
        # so we need to explicitly return -1 rather than just int(a) etc.
 
        return (int(b) if b>= 0 else -1, int(a) if a >= 0 else -1)
 
    def update_display(self,  debug_board= False):
        self.v.display(self.m.board,  debug_board = debug_board)
        self.v2.display(self.m.board, debug_board= debug_board)
 
    def parse_move(self, move):
        ''' Very basic move parsing
        given a move in the form ab-cd where a and c are in [a,b,c,d,e,f,g,h]
        and b and d are numbers from 1 to 8 convert into BoardLocation instances
        for start (ab) and destination (cd)
        Does not deal with castling (ie 0-0 or 0-0-0) or bare pawn moves (e4)
        or capture d4xe5 etc
        No error checking! very fragile
        '''
 
        s, d = move.split("-")
 
        i = 8- int(s[-1]) # board is "upside down" with reference to the representation
        j = column_reference.index(s[0])
        start = BoardLocation(i, j)
 
        i =  8- int(d[-1])
        j= column_reference.index(d[0])
        destination = BoardLocation(i, j)
 
        return start,  destination
 
if __name__=="__main__":
    missing_files = False
    if not os.path.exists(ISOMETRIC_DATA_DIR):
        missing_files = True
        print ("Cannot find data directory")
 
    if not missing_files:
        missing_list = []
        missing_2dlist=[]
        for k, v in ISOMETRIC_TILES.items():
            fn = os.path.join(ISOMETRIC_DATA_DIR,  v)
            if not os. path.exists(fn):
                missing_files = True
                print ("Cannot find file: %s"%fn)
                missing_list.append(v)
        for k, v in TILES.items():
            fn = os.path.join(ISOMETRIC_DATA_DIR,  v)
            if not os. path.exists(fn):
                missing_files = True
                print ("Cannot find file: %s"%fn)
                missing_2dlist.append(v)
 
    else: # whole directory missing
        missing_list= ISOMETRIC_TILES.values()
        missing_2dlist = TILES.values()
 
    if missing_files:
        ''' basic check - if there are files missing from the data directory, the
        program will still fail '''
        dl = input("Cannot find chess images directory.  Download from website? (Y/n)")
        if dl.lower() == "n":
            print("Some image files not found, quitting.")
            exit(0)
        if not os.path.exists(ISOMETRIC_DATA_DIR):
            print("Creating directory: %s"%os.path.join(os.getcwd(), ISOMETRIC_DATA_DIR))
            os.mkdir(ISOMETRIC_DATA_DIR)
 
        import urllib
        url_format="https://python4kids.files.wordpress.com/2013/05/chess_isometric_black_tile.gif"
        url_format= "https://python4kids.files.wordpress.com/2013/05/%s"
#        for k, v in ISOMETRIC_TILES.items():
        for v in missing_list:
            url = url_format%v
            target_filename = os.path.join(ISOMETRIC_DATA_DIR, v)
            print("Downloading file: %s"%v)
            urllib.request.urlretrieve(url, target_filename)
 
        url_format= "https://python4kids.files.wordpress.com/2013/04/%s"
        for v in missing_2dlist:
            url = url_format%v
            target_filename = os.path.join(ISOMETRIC_DATA_DIR, v)
            print("Downloading file: %s"%v)
            urllib.request.urlretrieve(url, target_filename)
    parent = tk.Tk()
    c = Controller(parent)
    c.run(debug_mode= False)