############################### day 01 ###############################
#%%
txt = open('2015_01.txt', encoding='utf-8').read()
sum(map(lambda x: -1 if x == ')' else 1, txt))

#%%
import numpy as np
arr = np.array([*map(lambda x: -1 if x == ')' else 1, txt)])
np.where(arr.cumsum() == -1)

############################### day 02 ###############################
#%%
txt = open('2015_01.txt', encoding='utf-8').read()
lines = txt.strip().split('\n')
def slack(values):
    s1, s2 = sorted(values)[:2]
    return s1 * s2

def area(line):
    l,w,h = [*map(int, line.split('x'))]
    return 2*l*w + 2*w*h + 2*h*l + slack([l,w,h])
sum(map(area, lines))
#%%
def perim(values):
    s1, s2 = sorted(values)[:2]
    return 2 * (s1 + s2)

def length(line):
    l,w,h = [*map(int, line.split('x'))]
    return perim([l,w,h]) + l*w*h

sum(map(length, lines))


############################### day 03 ###############################
#%%
from collections import defaultdict
txt = open('2015_03.txt', encoding='utf-8').read()
def deliver_presents(presents, txt):
  x,y = (0,0)
  for c in txt:
    dx, dy = 0, 0
    if c == '>': dx = 1
    elif c == '<': dx = -1
    elif c == '^': dy = 1
    elif c == 'v': dy = -1
    #
    x += dx
    y += dy
    presents[(x,y)] += 1
  return presents

############################### day 04 ###############################
#%%
presents = defaultdict(int)
presents[(0,0)] += 1
presents = deliver_presents(presents, txt)
len(presents.values())

#%%
presents = defaultdict(int)
presents[(0,0)] += 1
presents = deliver_presents(presents, txt[::2])
presents = deliver_presents(presents, txt[1::2])
len(presents.values())

############################### day 05 ###############################
#%%
from hashlib import md5
prefix = "iwrupvqb"
def search(prefix, goal):
  nchar = len(goal)
  n = 1
  while True:
    h = md5(f"{prefix}{n}".encode('utf-8')).hexdigest()
    if h[:nchar] == goal:
      return n
    n += 1

search(prefix, "00000")

#%%
search(prefix, "000000")
