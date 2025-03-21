from bisect import bisect_left,bisect_right

p: list[float] = [x  for x  in range(20)]
p.remove(5)
print(bisect_left(p, 5))
q = p[0:5]  + [5,5,5,5,5,5] + p[5:]

print(q)
q.insert(bisect_right(q,5), 5.5)
print(q)