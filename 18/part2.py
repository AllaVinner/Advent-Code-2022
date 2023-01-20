import numpy as np
from collections import defaultdict

with open('./18/input.txt', 'r') as f:
    lines = f.readlines()

def parse(line):
    return  tuple([int(i) for i in line.split(',')])

def get_neighbours(t):
    neig = []
    for i in range(len(t)):
        c = list(t)
        c[i] += 1
        neig.append(tuple([cc for cc in c]))
        c[i] -= 2
        neig.append(tuple([cc for cc in c]))
    return neig

def is_out_of_bounds(t, N):
    for tt in t:
        if t<0 or t >= N:
            return True
    return False

N = 20

index = defaultdict(list)
droplet = np.zeros((N,N,N), dtype=int)
regions = np.zeros((N,N,N), dtype=int)
max_region = 0

for line in lines:
    t = parse(line)
    droplet[t[0],t[1],t[2]] = 1
    regions[t[0],t[1],t[2]] = -1


for x in range(N):
    for y in range(N):
        for z in range(N):
            for n in get_neighbours((x,y,z)):
                if is_out_of_bounds(n, N):
                    continue
                if regions[n[0],n[1],n[2]] == 0 or










