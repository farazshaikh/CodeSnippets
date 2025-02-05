from typing import Generator, Optional
import random

def character_generator(input: str) -> Generator[str, None, None]:
    for char in input:
        yield char

def odd_iterator(l: list[int]) -> Generator[int, None, None]:
    for i in l:
        if i % 2 != 0:
            yield i


def odd_adapter(gen: Generator[int, None, None]) -> Generator[int, None, None]:
    for i in gen:
        if i % 2 != 0:
            yield i

def rand_generator(count: Optional[int]) -> Generator[int, None, None]:
    match count:
        case None:
            while True:
                yield random.randint(1, 100)
        case _:
            for _ in range(count):
                yield random.randint(1, 100)



def odd_randoms(): 
    for x in odd_adapter(rand_generator()):
         print(x)