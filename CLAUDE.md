Ok let's create a sort of clone of into the breach

'''
G = grass
S = single building
D = double building
F = forest
M = mountain
W = water
I = monster ingress
P = power generator
L = landmine
R = rocket
'''

Test map = '''G S S S S G G T
G G G G G G T T
M G G G G G D D
M T G G W G D M
T G G G G G G M
T G W G I T G M
G G G G W G G M
G T G G G I G T'''

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

isometric tiles can be found in assets/tiles. they should be diamond shaped, 120x60 px on a transparent rectangular background

the game is split in to three parts:

1. an 8x8 regular grid, using ratatui TUI crate drawn in the CLI, with multiple layers (terrain (e.g. "Test map"), terrain effects (poision gas clouds, radiation, etc), pieces, legal move overlay, cursor overlay, etc ), player/npc/enemy layer, visual effect layer etc etc.. this might be a multi dimensional array? not sure how best to structure it, but I want to be able to toggle different layers on and off for debug etc. this is the main logic

2. a visual representation of the grid, drawn using macroquad crate (latest version) using the isometric tiles, layered in roughly the order given above. when you click on a player/npc piece it should draw all the legal moves in a transparent layer (also represented in the cli tui grid in real time) this will have the main game loop, handle mouse/keyboard input, and eventually sound

3. the cli tui part should communicate with the ENGINE which is going to be a seperate sub crate/module, it should communicate via something similar, or take heavy inspiration from the UCI universal chess interface protocol. testing for legal moves should be part of the engine

as a simple MVP or POC it should contain all three of the above. inside of the macroquad window it should draw the map of isometric tiles, respecting alpha/transparency channel. the user should be able to click on a P/mech.png and it should highlight that "square" and then in another transparent color it should draw all the legal moves (gotten from the ENGINE). the mech can move up to three spaces. it can not move on top of mountains, tiles with mechs, or tiles with leapers. moving across trees or water reduces one move per tree or water tile.

after all of the mechs have moved, then it is the computer's turn, it will move each of the H or leaper.png up to three spaces with the same move rules. when all three leapers are done moving, it goes back to the player's turn and wait for them to manipulate the mechs with the mouse. this logic should be implemented using state machines, with lots of comments explaining what each part is doing.

The board is meant to loosely represent a chessboard, it is 8x8 and uses the same notation (internally) 12345678 abcdefgh so let's be cognizant of that. it also uses a UCI type interface with the engine when talking with the main game and ENGINE module. from that perspective programmers knowledgable about how chess programming works should immediatley understand the architecture.

after enough progress has been made, commit your progress as you go

You may need to write early automation to simulate mouse click/drag and screenshot capability to review your progress.