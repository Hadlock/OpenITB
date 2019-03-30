"""
Brendan Scott
Python4Kids.wordpress.com
November 2014

Chess -
attach a graphical interface to the sunfish chess engine

"""

import sunfish
import mvc2
import Tkinter as tk
import logging

fn  ="log_chess.log"  # Warning! Will delete any existing file called log_chess.log!
logging.basicConfig(filename= fn, filemode='w', level = logging.DEBUG, format= "%(levelname)s %(asctime)s %(funcName)s @%(lineno)d %(message)s")

SELECT_PIECE, MOVE_PIECE, WAIT_COMPUTER = range(3)

ALGEBRAIC_DATA=zip([0,1,2,3,4,5,6,7],[c for c in "hgfedcba"])
ALGEBRAIC_DICT = {}
for k,v in ALGEBRAIC_DATA:
    ALGEBRAIC_DICT[k] = v

logging.debug(ALGEBRAIC_DATA)
logging.debug(ALGEBRAIC_DICT)

def location_to_algebraic(board_location):
    return "%s%s"%(ALGEBRAIC_DICT[7-board_location.j],8-board_location.i)

# def move_to_algebraic(start, end):
#     return "%s%s"%(location_to_algebraic(start),location_to_algebraic(end))

class Model(object):
    def __init__(self, *args):
        self.pos_list= [sunfish.Position(*args)]
        self.legal_moves = []

    def move(self,i,j):
        pos=self.pos_list[-1]
        self.pos_list.append(pos.move((i,j)))
        return self.pos_list[-1]
        # Sunfish returns a sunfish.Position object to us

class NewController(mvc2.Controller):
    def __init__(self,  parent = None,  model = None, headless = False):
        self.headless = headless
        if model is None:
            self.m = Model(sunfish.initial, 0, (True,True), (True,True), 0, 0)
            #sunfish.Position(sunfish.initial, 0, (True,True), (True,True), 0, 0)
        else:
            self.m = model

        # logging.debug self.m.board
        self.v = mvc2.View(parent)
        ''' we have created both a model and a view within the controller
        the controller doesn't inherit from either model or view
        '''
        self.v.canvas.bind("<Button-1>",  self.handle_click)
        # this binds the handle_click method to the view's canvas for left button down

        self.clickList = []  #TODO: update to be sunfish pos not board location
        # I have kept clickList here, and not in the model, because it is a record of what is happening
        # in the view (ie click events) rather than something that the model deals with (eg moves).
        self.click_state = SELECT_PIECE
        self.update_model()

    def update_model(self):
        self.m.legal_moves = [m for m in self.m.pos_list[-1].genMoves()]
        self.m.valid_clicks = set([m[0] for m in self.m.pos_list[-1].genMoves()])

    def update_display(self,  debug_board=True, board=None):
        if self.headless:
            return
        if board is None:
            self.v.display(self.convert(self.m.pos_list[-1].board),  debug_board = debug_board)
        else:
            self.v.display(self.convert(board))
        self.v.canvas.update_idletasks()
        # sometimes tkinter is stalling, so flush instructions

    def convert(self, board):
        """
        Sunfish adds blank lines, spaces and carriage returns to its board for its processing
        however the code we are using to display this assumes that board
        is an array of string.  So, convert
        Also need to convert "." representing an empty square to " " for the current view
        :param board:
        :return:
        """

        b = board.split('\n')
        converted_board = []
        for line in b[2:-2]:  # sunfish has two blank lines at top and bottom
            if line[0]== " ":
                line = line[1:].replace("."," ")
                # sunfish also has left padding sometimes
            else:
                line = line.replace("."," ")
            converted_board.append([c for c in line])  # split each line
        return converted_board

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

        logging.debug("in handle click valid moves are: \n%s"%self.m.legal_moves)
        j = event.x/mvc2.TILE_WIDTH
        #  the / operator is called integer division
        # it returns the number of times TILE_WIDTH goes into event.x ignoring any remainder
        # eg: 2/2 = 1, 3/2 = 1, 11/5 = 2 and so on
        # so, it should return a number between 0 (if x < TILE_WIDTH) though to 7
        i = event.y/mvc2.TILE_WIDTH

        b_loc = mvc2.BoardLocation(i, j)

        # Select/deselect logic

        sunfish_pos =  sunfish.parse(location_to_algebraic(b_loc))
        logging.debug("received a click at %s"%sunfish_pos)

        if self.click_state == WAIT_COMPUTER:
            logging.debug("wait computer: Currently waiting for computer ignoring clicks")
            return

        elif self.click_state == SELECT_PIECE:
            logging.debug("valid clicks are: %s"%self.m.valid_clicks)
            if sunfish_pos in self.m.valid_clicks:
                self.clickList.append(sunfish_pos)
                logging.debug("select piece: piece selected at: %s"%(sunfish_pos))
                self.click_state = MOVE_PIECE
                self.m.valid_clicks = [m[1] for m in self.m.legal_moves if m[0] == sunfish_pos]
                logging.debug("valid clicks now: %s"%self.m.valid_clicks)
                return
            else:
                return

        else: #State is MOVE_PIECE
            logging.debug("move piece: valid clicks are: %s"%self.m.valid_clicks)
            if sunfish_pos == self.clickList[-1]:
                logging.debug("piece moved to: %s"%(sunfish_pos))
                # that is, the currently selected piece
                # unselect that piece
                self.clickList.pop()
                self.click_state = SELECT_PIECE
                self.m.valid_clicks = set([m[0] for m in self.m.legal_moves])
                return

            elif sunfish_pos not in self.m.valid_clicks:
                # wait for a valid move
                return
            else:
                self.clickList.append(sunfish_pos)
                self.click_state = WAIT_COMPUTER
                # execute the move:
                pos = self.m.move(self.clickList[-2], self.clickList[-1])
                logging.debug(repr(pos.board))
                logging.debug("about to update display")
                self.update_display(board=pos.rotate().board)
                logging.debug("After updating display")
                self.query_sunfish(pos)

    def query_sunfish(self, pos):
        logging.debug("in query sunfish")
        move, score = sunfish.search(pos)

        if score <= -sunfish.MATE_VALUE:
            logging.debug("You won")

        if score >= sunfish.MATE_VALUE:
            logging.debug("You lost")

        logging.debug("move = %s->%s"%move)
        pos= self.m.move(*move)
        self.update_display(board=pos.board)
        logging.debug(pos.board)
        self.click_state=SELECT_PIECE
        self.update_model()

if __name__ == "__main__":
    parent = tk.Tk()
    c = NewController(parent)
    c.run(debug_mode= False)