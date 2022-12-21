import numpy as np
##############################################################################
# Read Data
##############################################################################
with open('./17/input.txt', 'r') as f:
    line = f.readline()

##############################################################################
# Setup shapes
##############################################################################

def move_side(direction, current_shape, shape_pos, grid):
    for p in current_shape:
        x = p[0]+shape_pos[0]+direction
        if x < 0 or 7 <= x:
            return shape_pos
        if grid[x,p[1]+shape_pos[1]] == 1:
            return shape_pos
    shape_pos[0] += direction
    return shape_pos

def move_down(current_shape, shape_pos, grid):
    for p in current_shape:
        y = p[1]+shape_pos[1]-1
        if grid[p[0]+shape_pos[0], y] == 1:
            return (shape_pos, True)
    shape_pos[1] -= 1
    return (shape_pos, False)

def fill_in_grid(current_shape, shape_pos, grid):
    for p in current_shape:
        grid[p[0]+shape_pos[0],p[1]+shape_pos[1]] = 1

def get_top(current_shape, shape_pos, top):
    for p in current_shape:
        if p[1]+shape_pos[1] > top:
            top = p[1]+shape_pos[1]
    return top

def spawn_new_shape(shape_i, top):
    current_shape = shapes[shape_i % len(shapes)]
    shape_pos = [2, top + 4]
    return (current_shape, shape_pos)


minus_shape = [[0,0],[1,0],[2,0],[3,0]]
plus_shape = [[1,0],[0,1],[1,1],[2,1], [1,2]]
L_shape = [[0,0],[1,0],[2,0],[2,1], [2,2]]
l_shape = [[0,0],[0,1],[0,2],[0,3]]
square_shape = [[0,0],[1,0],[1,1],[0,1]]

shapes = [minus_shape, plus_shape, L_shape, l_shape, square_shape]


grid = np.zeros((7, 2022*4), dtype = int)
grid[:, 0] = 1
top = 1
shape_pos = [2, 4]
shape_i = 0
current_shape = shapes[shape_i % len(shapes)]

while (True):
    breaking = False
    for direction in map(lambda c: 1 if c=='>' else -1, line):
        shape_pos = move_side(direction, current_shape, shape_pos, grid)
        (shape_pos, collided) = move_down(current_shape, shape_pos, grid)
        if collided:
            fill_in_grid(current_shape, shape_pos, grid)
            top = get_top(current_shape, shape_pos, top)
            shape_i += 1
            if shape_i > 2021:
                breaking = True
                break
            (current_shape, shape_pos) = spawn_new_shape(shape_i, top)
    if breaking:
        break

print(top)

