from collections import ChainMap, namedtuple



p = dict([(x,str(x)) for x in range(10)])
q = dict([(x, str(x)) for x in range(14, 30)])
print(p)
print(q)

m: dict[int, str] = dict()
m[1] = "1"
print(f"m ={m}")

p[10] = "Ten"
q[10] = "10"
combined = ChainMap(p,q)
print(combined.get(10, "Not found"))


Person = namedtuple("Person", [ "personalItem", "name" ])
p  = Person(personalItem=[], name="mutabl")
p.personalItem["cars"] = ["BMW" , "Honda"]
p = p._replace(name="faraz")



