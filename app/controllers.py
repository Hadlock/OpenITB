#/bin/python3
import tkinter as tk
from .models import Model
from .models import BoardLocation
from .views import Isometric_View
from .views import View

column_reference = "a b c d e f g h".split(" ")
TILE_WIDTH = 60
ISOMETRIC_TILE_WIDTH = 120
ISOMETRIC_TILE_HEIGHT = 60

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