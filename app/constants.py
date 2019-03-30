#/usr/bin/python3
'''this holds all global constants'''

COLUMN_REFERENCE = "a b c d e f g h".split(" ")
EMPTY_SQUARE = " "

TILE_WIDTH = 60

ISOMETRIC_TILE_WIDTH = 120
ISOMETRIC_TILE_HEIGHT = 60

#ISOMETRIC_TILE_WIDTH = 58
#ISOMETRIC_TILE_HEIGHT = 42


# to display these tiles many locations in the code rely on integer division by 2,
# so this width and height should both be even numbers (otherwise rounding errors will accumulate)

BOARD_WIDTH = 8*TILE_WIDTH
BOARD_HEIGHT = BOARD_WIDTH

ISOMETRIC_BOARD_WIDTH = 8 * ISOMETRIC_TILE_WIDTH
ISOMETRIC_BOARD_HEIGHT = 8*ISOMETRIC_TILE_HEIGHT

DATA_DIR = "chess_data"
ISOMETRIC_DATA_DIR = "chess_data"

ISOMETRIC_TILES = {# "black_tile":"chess_isometric_black_tile.gif",
                   "black_tile":"water.png",
                   "white_tile":"mountain.png",
                   "B":"chess_b451.gif",
                   "b":"chess_b45.gif",
                   "k":"chess_k45.gif",
                   "K":"chess_k451.gif",
                   "n":"chess_n45.gif",
                   "N":"chess_n451.gif",
                   "p":"chess_p45.gif",
                   # "P":"chess_p451.gif",
                   # "P":"chess_isometric_white_pawn2.gif",
                   "P":"mech.png",
                   "q":"chess_q45.gif",
                   "Q":"chess_q451.gif",
                   "r":"chess_r45.gif",
                   "R":"chess_r451.gif",
                   # "white_tile":"chess_isometric_white_tile.gif"
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
