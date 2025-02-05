def main() -> None:
    # Your code here
    a = [*reversed(range(1,6))]
    a.extend(range(1,6))
    for i,v in enumerate(set(a)):
        print(f"Index: {i} Value: {v}")
    pass

if __name__ == "__main__":
    main()