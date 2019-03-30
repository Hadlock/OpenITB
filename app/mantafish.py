#/usr/bin/python3
'''primary game logic'''

def get_empty_spaces(pieces, maptiles):
    '''compare pieces, maptiles, then find empty
    spaces where units can move'''

    # TODO: implement this
    empty_spaces = []
    for j, row in enumerate(pieces):
        tile = ""
        for i, piece in enumerate(reversed(row)):
            print(piece)
            if piece in '.':
                if maptiles[j][i] in ('G', 'T', 'W', 'I'):
                    # this is safe
                    tile+='. '
            else:
                # cannot move here
                tile+='X '
        empty_spaces.append(tile.split(" "))

def get_move_range(start, empty_spaces, range):
    '''given a start location, empty_spaces and range,
    find legal moves and return them as a list of
    coordinates'''
    # TODO: implement this
    legal_moves = []
    for r in range:
        if empty_spaces:
            print("hello world")
    return legal_moves

def find_adjacent_targets(legal_moves, pieces, maptiles):
    '''given a list of legal moves, find targets
    to attack within one square,
    return a dict of move_coordinates, target_coordinates'''
    # TODO: implement this
    adjacent_targets = {}
    return adjacent_targets