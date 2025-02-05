from typing import List
import random



# sort arry 0 to end + 1
# l h inclusive
# objective to place pivot in its right location
def partition_h(i: List[int], l: int, h:int):
    p: int  = i[h]
    while l < h:
        if i[l] < p:
            l=l+1
            continue
        if i[h] > p:
            h = h-1
            continue
        i[h], i[l] = i[l], i[h]
    return l

def qs_h(i: List[int], l: int, h:int):
    if l >= h:
        return
    
    p = partition_h(i,l,h)
    qs_h(i, l, p-1)
    qs_h(i, p, h)

def quicksort(i: List[int]):
    if len (i) < 2:
        return
    qs_h(i, 0, len(i) - 1) 

def issorted(ip: List[int]) -> bool:
    for i in range(0,len(ip)-1):
        if ip[i] > ip[i+1]:
            return False
    return True
    



def permutate(ip:List[int]) -> List[List[int]]:
    if len(ip) == 0:
        return []
    if len(ip) == 1:
        return [ip]
    
    perms: List[List[int]] = []
    # keep first constant
    for i in range(0,len(ip)):
        ip[0],ip[i] = ip[i],ip[0]
        
        sub_perms: List[List[int]] = permutate(ip[1:])
        for s in sub_perms:
           perms.append([ip[0]] + s)
        
        #perms.extend(sub_perms)
        ip[0],ip[i] = ip[i],ip[0]

    return perms

def factorial(n: int) -> int:
    if n == 0:
        return 0
    a = 1
    for i in range(1,n+1):
        a = a * i
    return a

print("Module initialized")



if __name__ == '__main__':  
    for input_len in range(0,7):
        l: List[int] = [*range(0,input_len)]
        p = permutate(l);
        assert len(p) == factorial(len(l)) 

        for x in permutate(l):
            print(f"Sorting {x}")
            quicksort(l)
            assert issorted(l)
