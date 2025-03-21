import collections

class Stack:
	q: collections.deque

	def __init__(self) -> None:
		self.q = collections.deque()

	def push(self, val: int):
		self.q.append(val)

	def pop(self) -> int:
		return self.q.pop()


s = Stack()
for x in range(10):
	s.push(x)

print(s.q)
assert(s.pop() == 9)
assert(s.pop() == 8)
print(s.q)


q: collections.deque = collections.deque()
