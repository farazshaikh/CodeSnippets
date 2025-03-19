import collections
import dataclasses
@dataclasses.dataclass
class Student:
    name: str
    num: int

class LRUCache:
    entries: collections.OrderedDict
    max_elements: int

    def __init__(self, max_elements: int):
        self.entries = collections.OrderedDict()
        self.max_elements = max_elements


    def insert(self, value: int):
        self.entries[value] = ""
        # [H .... T] tail tracks the most recent
        self.entries.move_to_end(value)
        while len(self.entries) > self.max_elements:
            self.entries.popitem(last=False)

    def __repr__(self):
       return self.entries.__repr__()


def main():
    cache = LRUCache(3);
    for i in range(3):
        cache.insert(i)
    print(cache)
    cache.insert(0)
    print(cache)
    cache.insert(4)
    print(cache)

if __name__ == "__main__":
    main()