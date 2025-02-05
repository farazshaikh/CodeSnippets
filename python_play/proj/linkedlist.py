from typing import Self, Optional



class Node:
    data: int
    next: Optional[Self]

    def __init__(self, data=int, /)-> None:
        self.data = data
        self.next = None

    def prepend(self, data=int) -> Self:
        node = Node(data)
        node.next = self
        return node
    
    def traverse(self) -> None:
        trav: Self = self
        while trav:
            print(f"{trav.data}")
            trav = trav.next
    
    def __rec_rev__(self) -> tuple[Self, Self]:
        assert self != None
        if self.next == None:
            return self, self
        
        thisnode = self
        (firstnode, lastnode) = Node.__rec_rev__(self.next)
        lastnode.next = thisnode
        thisnode.next = None
        lastnode = thisnode
        return (firstnode, lastnode)


    def rev(self) -> Self:
        if self == None:
            return self
        (first, _) = Node.__rec_rev__(self)
        return first
    
    def pop_head(self) -> tuple[int, Self]:
        if self == None:
            return None, None
        data = self.data
        self.next
        return data, self.next 
    
class LinkedList:
    head: Optional[Node]
    
    def __init__ (self) -> Self: 
        self.head = None

    def isEmpty(self) -> bool:
        return self.head == None

    def prepend(self, data=int) -> Self:
        if self.head == None:
            self.head = Node(data)
        else:
            self.head = self.head.prepend(data)

    def traverse(self) -> None:
        if self.head:
            self.head.traverse()
        return 

    def rev(self) -> None:
        if self.head:
            self.head  = self.head.rev()  

    def pop_head(self) -> Optional[int]:
        if self.head:
            data, head = self.head.pop_head() 
            self.head = head 
            return data
        return None


if __name__ == "__main__":
    ll = LinkedList()
    for x in range(1,11):
        ll.prepend(x)

    print("Orginal list")
    ll.traverse()

    print("Reversedlist")
    ll.rev() 
    ll.traverse()

    while ll.isEmpty() == False:
        data = ll.pop_head()
        print(f"{data}")
