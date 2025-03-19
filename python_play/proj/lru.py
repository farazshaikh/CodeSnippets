import collections

class LRUCache:
    k: collections.OrderedDict
    def __init__(self) -> None:
        self.k = collections.OrderedDict()