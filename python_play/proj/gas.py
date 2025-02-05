from typing import List


class Solution:

    def collapse(self, x):
        x = x[0]
        l = len(x)
        if l == 1:
            return False
        # collapse las with first
        i = 0
        collapsed = False
        while True:
            if i == len(x):
                break

            #collapse forward
            if x[i][1] >= 0:
                x[(i+1) % l][1] += x[i][1]
                x[(i+1) % l][0] = x[i][0]
                del x[i]
                collapsed = True
            else: 
                i+=1
           
        return collapsed

    def canCompleteCircuit(self, gas: List[int], cost: List[int]) -> int:
        summary  = []
        l = len(gas)
        for i in range(0,l):
            summary.append([i, gas[i]  -cost[i]])

        print(f"Summary {summary}")

        while True:
            collpased= self.collapse([summary])
            print(f"c summary {summary}")
            if not collpased:
                break

        print(summary)
        if len(summary) > 1:
            return -1

        if summary[0][1] >= 0:
            return summary[0][0]

        return -1  
    

x = Solution().canCompleteCircuit([1,2,3,4,5],[3,4,5,1,2])
print(x)