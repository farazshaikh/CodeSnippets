from typing import Any
def filter_list(lst: list[Any]) -> list[Any]:
    ret: list[Any] = []
    for i in lst:
        if i != None:
            ret.append(i)

    return ret
    pass

def main():
    x = [None if i % 2 == 0 else i for i in range(0, 11)]
    print(x)
    filtered_x = filter_list(x)
    print(filtered_x)
    x = [x for x in range(0,10)]
    print("Hello, World!")

if __name__ == "__main__":
    x:int = 6
    if x < 10 and x % 2 == 0:
        print(f"{x} is even and less than 10")
    main()