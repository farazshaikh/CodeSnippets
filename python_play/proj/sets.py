def main():
    # Your code here
    a : set = {1,2,3,4,5}
    b : set = {4,5,6,7,8}
    print(a.intersection(b))
    print(a.union(b))
    print(a.difference(b))
    print(a.symmetric_difference(b))
    print(a.issubset(b))
    print(a.issuperset(b))
    print(a.isdisjoint(b))
    pass

if __name__ == "__main__":
    main()