
from abc import abstractmethod
from dataclasses import dataclass
from enum import Enum
import random
from typing import Generator, Literal
class Object:
    pass

@dataclass
class Student:
    name: str
    id: int
    age: int

def create_student(count: int) -> list[Student]:
    return[
                Student(f("{i} student"), id, random.randrange(10, 20))
                for id in range(count)
          ]

class Shape:
    @abstractmethod
    def draw(self):
        print("Drawinging generic shape")
    pass

class Circle(Shape):
    def draw(self):
        print("Drawinging Circle shape")
    pass

class Triangle(Shape):
    def draw(self):
        print("Drawinging Triangle shape")
    pass


def generate_next_in_sequence() -> Generator[int | Literal['faraz'], random.Any, Literal['I am done']]:
    yield int(1)
    yield int(2)
    yield int(3)
    yield int(4)
    yield "faraz"
    return "I am done"



def create_random_shapes() -> list[Shape]:
    return [
            random.choice([Circle(), Triangle()])
            for _ in range(10)
            ]

class ShapeTyoes(Enum):
    CIRCLE = 1
    TRIANGLE = 2

class MyClass:
    private =1
    def __init__(self, x):
        self.x = x
        self.y = 10
        self.private = 100

    def __str__(self):
        return f"MyClass({self.x})" 
    
    def non_method(i: int):
        print(f"non method {i}")
        return i

    def method(self, i: int):
        print(f"method {i} {self.x} {self.y}")
        return i 

    def print_private(self):
        print(f"In print private method {self.private}")   

    def print_private():
        print(f"In print private class helper {MyClass.private}")


print(MyClass.print_private.__annotations__)

shapes = create_random_shapes()
for shape in shapes:
    shape.draw()

# MyClass.non_method(1)
# obj = MyClass(1)
# obj.method(1)
# print(MyClass.private)
# obj.print_private()

# MyClass.private = 1000
# obj.print_private()
# obj2 = MyClass(100)
# obj2.print_private()

my_class = MyClass(1);
f = my_class.method
del my_class
f(1)