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

import os.path
import tkinter as tk
from tkinter import PhotoImage
from app.models import Model
from app.models import BoardLocation
from app.views import View
from app.views import Isometric_View
from app.controllers import Controller

column_reference = "a b c d e f g h".split(" ")
EMPTY_SQUARE = " "

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