#/usr/bin/python3
''' first attempt at handling maps'''
# pylint: disable=C0103
# pylint: disable=W0105

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

forgotten_hills = '''G S S S S G G T
G G G G G G F T
M G G G G G D D
M T G G W G D M
T G G G G G G M
T G W G I T G M
G G G G W G G M
G F G G G I G T'''

archivist_hall = '''M M G G G S W W
M S G S G T M W
G G T D G G M W
G D G G G G M W
G P G G G T M W
G G G G I G G W
G G I G G T G W
G T G T G G T W'''

storage_vaults = '''W W T G G S M M
W S G T G S S M
W S G F G G F G
W G G G G G G G
W D T G T D G G
W D G G G D G G
W W G G F I G G
W W I G G G G M'''

central_museums = ''''G G G G T M M M
T S S G G S M M
T D D G T D S M
G G G G G G T G
G G D G G T R T
M G G G G I G T
M G G I R G T T
M M G T T T T T'''

martial_district = '''M M G G G G G G
M S T L L P S T
M M G G G M S G
M M G L T L G T
M D T L L T D M
G G G L T T M M
G L I G G I D M
M G G M I G M M'''

secondary_archives = '''M M G S S G M M
M G G G T G S S
G G P G G G G D
G T D G G R G G
G G G G G W W G
G G G T I W W W
G G G I G G G G
G G T G G G G G'''

relic_preserves = '''M G S S G G G G
M G S S G G W G
G G G G G P M G
T D W G G G G G
G W W G G D G G
G D I G G W W G
T G G G G G G G
G G G M I G G G'''
