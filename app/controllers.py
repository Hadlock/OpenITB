#/bin/python3
''' controller for openitb'''
import tkinter as tk
from .models import Model
from .models import BoardLocation
from .views import IsometricView
from .constants import COLUMN_REFERENCE, TILE_WIDTH, ISOMETRIC_TILE_HEIGHT

class Controller(object):
    ''' primary controller '''
    def __init__(self, parent=None, model=None):
        if model is None:
            self.model = Model()
        else:
            self.model = model
        self.view = IsometricView(parent)

        # disabling the overhead view for now
        # leaving it wired up, just in case, for debug
        # self.view2 = View(parent)

        ''' we have created both a model and a view within the controller
        the controller doesn't inherit from either model or view
        '''
        self.view.canvas.bind("<Button-1>", self.handle_click_isometric)
        # this binds the handle_click method to the view's canvas for left button down
        # self.view2.canvas.bind("<Button-1>",  self.handle_click)
        self.click_list = []
        # I have kept click_list here, and not in the model, because it is
        # a record of what is happening in the view (ie click events) rather
        # than something that the modelmdeals with (eg moves).

    def run(self, debug_mode=False):
        ''' initializes tk mainloop'''
        self.update_display(debug_board=debug_mode)
        tk.mainloop()

    def handle_click(self, event):
        ''' Handle a click received.  The x,y location of the click on the canvas is at
        (event.x, event.y)
        First, we need to translate the event coordinates (ie the x,y of where the click occurred)
        into a position on the chess board
        add this to a list of clicked positions
        every first click is treated as a "from" and every second click as a"to"
        so, whenever there are an even number of clicks, use the most recent to two to perform
        a move then update the display
        '''
        j = event.x//TILE_WIDTH
        #  the / operator is called integer division
        # it returns the number of times TILE_WIDTH goes into event.x ignoring any remainder
        # eg: 2/2 = 1, 3/2 = 1, 11/5 = 2 and so on
        # so, it should return a number between 0 (if x < TILE_WIDTH) though to 7
        i = event.y//TILE_WIDTH

        self.click_list.append(BoardLocation(i, j))
        # just maintain a list of all of the moves
        # this list shouldn't be used to replay a series of moves because that is something
        # which should be stored in the model - but it wouldn't be much trouble to
        # keep a record of moves in the model.
        if len(self.click_list)%2 == 0:
            # move complete, execute the move
            self.model.move(self.click_list[-2], self.click_list[-1])
            # use the second last entry in the click_list and the last entry in the click_list
            self.update_display()

    def handle_click_isometric(self, event):
        ''' Handle a click received.  The x,y location of the click on the canvas is at
        (event.x, event.y)
        First, we need to translate the event coordinates (ie the x,y of where the click occurred)
        into a position on the chess board
        add this to a list of clicked positions
        every first click is treated as a "from" and every second click as a"to"
        so, whenever there are an even number of clicks, use the most recent to two to perform
        a move then update the display
        '''
        # i, j represent the X-Y coordinates on the board in chess notation, assuming a 0 index
        i, j = self.xy_to_ij(event.x, event.y)
        print("event x: " + str(event.x) + " y: " + str(event.y))
        print("and i: " + str(i) + " j: " + str(j))
        self.click_list.append(BoardLocation(7-i, j))  # 7-i because the Model stores the board in
        # reverse row order. just maintain a list of all of the moves
        # this list shouldn't be used to replay a series of moves because that is something
        # which should be stored in the model - but it wouldn't be much trouble to
        # keep a record of moves in the model.
        if len(self.click_list)%2 == 0:
            # move complete, execute the move
            self.model.move(self.click_list[-2], self.click_list[-1])
            print(str(self.click_list[-2]) + " ; " + str(self.click_list[-1]))
            # use the second last entry in the click_list and the last entry in the click_list
            self.update_display()

    def xy_to_ij(self, x, y): # pylint: disable=C0103
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

        y = self.view.canvas_height-y # invert it
        print("y1: " + str(y))
        y = y - 4*ISOMETRIC_TILE_HEIGHT # Get y relative to the height of the corner
        print("y2: " + str(y))
        a = (x-2*y)/120.0 # this calculates the 'X' in the model from gui # pylint: disable=C0103
        print("a: " + str(a))
        b = (y+30*a)/30.0 # this calculates the 'Y' in the model from gui # pylint: disable=C0103
        print("b: " + str(b))
        # if either of these is <0 this means that the click is off the board (to the left or below)
        # if the number is greater than -1, but less than 0, int() will round it up to 0
        # so we need to explicitly return -1 rather than just int(a) etc.
        return (int(b) if b >= 0 else -1, int(a) if a >= 0 else -1)

    def update_display(self, debug_board=False):
        ''' updates the display '''
        self.view.display(self.model.board, debug_board=debug_board)
        # self.view2.display(self.model.board, debug_board= debug_board)

    def parse_move(self, move):
        ''' Very basic move parsing
        given a move in the form ab-cd where a and c are in [a,b,c,d,e,f,g,h]
        and b and d are numbers from 1 to 8 convert into BoardLocation instances
        for start (ab) and destination (cd)
        Does not deal with castling (ie 0-0 or 0-0-0) or bare pawn moves (e4)
        or capture d4xe5 etc
        No error checking! very fragile
        '''

        s, d = move.split("-") # pylint: disable=C0103

        i = 8- int(s[-1]) # board is "upside down" with reference to the representation
        j = COLUMN_REFERENCE.index(s[0])
        start = BoardLocation(i, j)

        i = 8- int(d[-1])
        j = COLUMN_REFERENCE.index(d[0])
        destination = BoardLocation(i, j)

        return start, destination
