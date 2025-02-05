def delall_retain(l, n):
    out = []
    for x in l:
        if x != n:
            out.append(x)
    return out

def control(n):
    if n == 0:
        print("zero")
    elif n == 1:
        print("one")
    elif n == 2:
        print("two")
    else:
        print("greater than two or less than zero")

def fibonacci(n):
    a, b =  0, 1
    i = 0
    while i < n:
        print(a, end=",OA")
        a, b = b, a + b
        i = i + 1

def flatten(l):
    out = []
    for item in l:
        if isinstance(item, (list, tuple)):
            out.extend(flatten(item))
        else:
            out.append(item)
    return out

def isPrime(n):
    if n == 0:
        print("zero neither prime nor composite")
        return
    elif n == 1:
        print("{n} is prime")
        return

    for i in range(2, n//2):
        if n % i == 0:
            print(f"{n} is not prime found divisor {i} ")
            break
    else:
        print(f"{n} is prime")

def emptyfunction():
    1+1

def printbyname(x):
    s = ""
    match x:
       case 1:
           s = "one"
       case 2:
           s  = "two"
       case 3:
           s = "three"
       case 5|6|7:
           s  = "fix or six or seven"
       case _:
           s = "Less than 0 or greater 3"
    print(s)

class Point:
    __match_args__ = ('x', 'y')
    def __init__(self, x,y):
        self.x  = x
        self.y = y

def pointtype(point):
    match point:
        case Point(x=0, y=0):
            print("Origin")
        case Point(x=0, y=y):
            print(f"Y intercept  = {y} aka f(0) = {y}")
        case Point(x=x, y=0):
            print(f"X intercept = {x} aka f({x})= 0")
        case Point(x,y) if x > y:
            print(f"Evaluation f{x} = {y} x > y")
        case Point(x,y):
            print(f"Evaluation f{x} = {y} x <= y")
        case _:
            print("not a pointOA")

def main():
    #print(flatten([1, [2, 3, [4, 5], 6], 7, 8]))
    fibonacci(10)

    print("control flow")

    control(0)
    control(1)
    control(4)

    w = ["this", "is", "a"]
    for word in w:
        print(word)




main()
