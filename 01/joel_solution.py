import requests

##############################################################################
# part 1
##############################################################################

link = "https://adventofcode.com/2022/day/1/input"
session = '53616c7465645f5fda0b6e3b208740b7abf10d247a7adff161b4fe780a6e764ac562eb5796ad05a72bae25239ce755d8060daee6fc2629bef60ca76a20fb788c'


# Use 'with' to ensure the session context is closed after use.
s = requests.Session()
r = s.get(link, cookies={'session': session})

l = [[int(ss) for ss in s.split('\n') if ss not in [None, '']] for s in r.text.split('\n\n')]
sums = [sum(ll) for ll in l]
max(sums)


##############################################################################
# part 2
##############################################################################

sums.sort()

sums[-1]+sums[-2]+sums[-3]

