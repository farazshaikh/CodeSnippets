import os
from dataclasses import dataclass, asdict
import json

@dataclass
class Student:
    rollnumber: int
    name: str



def write_to_file(s: list[Student], file_path: str):
    fd: int = os.open(file_path, os.O_WRONLY | os.O_CREAT)
    sd = [st.__dict__ for st in s]
    os.write(fd, json.dumps(sd).encode('utf-8'))
    os.close(fd)

def read_from_file(file_path: str) -> list[Student]:
    fd  = os.open(file_path, os.O_RDONLY)
    st = os.fstat(fd)
    data = os.read(fd, st.st_size)
    os.close(fd)
    ret: list[Student] =  json.loads(data.decode('utf-8'))
    return ret


write_to_file([ Student(i, format("Student-{i}") )  for i in range(10)], "/tmp/foo.txt")
k = read_from_file("/tmp/foo.txt")
print(k)
