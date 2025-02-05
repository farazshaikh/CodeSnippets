import pprint
import reprlib
s = set("abscedfghikjlmnopqurstuvwxyzabcdefghikj")
p = list(s)
p = list([*range(10000)])
p.sort()
print(reprlib.repr(p))
pprint.pprint(p)


text =  "An an valley indeed so no wonder future nature vanity. Debating all she mistaken indulged believed provided declared. He many kept on draw lain song as same. Whether at dearest certain spirits is entered in to. Rich fine bred real use too many good. She compliment unaffected expression favourable any. Unknown chiefly showing to conduct no. Hung as love evil able to post at as.\
\
Neat own nor she said see walk. And charm add green you these. Sang busy in this drew ye fine. At greater prepare musical so attacks as on distant. Improving age our her cordially intention. His devonshire sufficient precaution say preference middletons insipidity. Since might water hence the her worse. Concluded it offending dejection do earnestly as me direction. Nature played thirty all him.\
\
Man request adapted spirits set pressed. Up to denoting subjects sensible feelings it indulged directly. We dwelling elegance do shutters appetite yourself diverted. Our next drew much you with rank. Tore many held age hold rose than our. She literature sentiments any contrasted. Set aware joy sense young now tears china shy.\
\
Be at miss or each good play home they. It leave taste mr in it fancy. She son lose does fond bred gave lady get. Sir her company conduct expense bed any. Sister depend change off piqued one. Contented continued any happiness instantly objection yet her allowance. Use correct day new brought tedious. By come this been in. Kept easy or sons my it done.\
\
Living valley had silent eat merits esteem bed. In last an or went wise as left. Visited civilly am demesne so colonel he calling. So unreserved do interested increasing sentiments. Vanity day giving points within six not law. Few impression difficulty his use has comparison decisively."


import textwrap

print(textwrap.fill(text, width=40))

from string import Template

t = Template("ping ${ip_addr} resolved to ${dns}")
m= t.substitute(ip_addr='127.0.0.1', dns='localhost')
m  = dict(ip_addr='192.168.1.1', dns='home_router' )
m = t.substitute(m)


print(m)

for ids in t.get_identifiers():
    print(ids)
    

class CustomTemplate(Template):
    delimiter = '%'
    
t = CustomTemplate("This is a customtemplate by author %name")
print(t.substitute(name="Faraz"))


from threading import Semaphore
import threading
from time import sleep
import logging

val = 0 
lm  = [Semaphore(0) for _ in range(3)]
def threadRelay(self_nr: int, l: list[Semaphore], max_ct:int=5):
    global val
    while val < max_ct:
        l[self_nr].acquire()
        logging.critical(f"Thread {self_nr}")
        print(f"Thread {self_nr} printing {val}")
        val += 1
        sleep(1)    
        l[(self_nr+1) % len(l)].release() 

lm[0].release()
threads = [threading.Thread(target=threadRelay, args=[x, lm]) for x in range(3) ] 


for th in threads:
    th.start()
    
for th in threads:
    th.join()
