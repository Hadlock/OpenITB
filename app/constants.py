#/usr/bin/python3
'''this holds all global constants'''

COLUMN_REFERENCE = "a b c d e f g h".split(" ")
EMPTY_SQUARE = "."

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

DATA_DIR = "app/tiles"
ISOMETRIC_DATA_DIR = "app/tiles"

ISOMETRIC_TILES = {"black_tile":"black_tile.gif",
                   "white_tile":"black_tile.gif",
                   "P":"mech.png",
                   "G":"ground.gif",
                   "S":"double.png",
                   "D":"double.png",
                   "T":"trees.png",
                   "M":"mountain.png",
                   "W":"water.png",
                   "I":"ground.gif",
                   "H":"leaper.png"
                   }

TILES = {"black_tile":"black_tile.gif",
         # leaving this here for possible future debug
         "white_tile":"white_tile.gif"
         }
