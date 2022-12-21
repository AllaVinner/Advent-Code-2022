

##############################################################################
# Read Data
##############################################################################
with open('./18/input.txt', 'r') as f:
    lines = f.readlines()

def parse(line):
    return  tuple([int(i) for i in line.split(',')])

def get_neighbours(x):
    neig = []
    for i in range(len(x)):
        c = list(x)
        c[i] += 1
        neig.append(tuple([cc for cc in c]))
        c[i] -= 2
        neig.append(tuple([cc for cc in c]))
    return neig

visited = {}

sides = 0

for line in lines:
    x = parse(line)
    sides += 6
    visited[x] = 1
    for n in get_neighbours(x):
        if visited.get(n, None) == 1:
            sides -= 2

sides

get_neighbours((0,3))