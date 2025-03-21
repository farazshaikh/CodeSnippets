import heapq
import random

def heapsort(elt: list[int]):
    m : list[int] = [random.randint(0, 100) for _ in range(10000) ]
    heapq.heapify(m)
    n = [heapq.heappop(m) for _ in range(len(m))]
    print(n)

heapsort([])