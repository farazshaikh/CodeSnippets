from typing import List

class Solution:
    # 1 2 3 4 3 2 1
    # 2 3 5 6 5 4 3 1
    # 0 1 2 3 4 5 6 7 8 9

    # 1 2 3 4 3 2 1 
    # 2 3 5 6 5 4 3 2 1
    # 0 1 2 3 4 5 6 7 8 9


    def candy(self, ratings: List[int]) -> int:
       
        l = len(ratings)
        last_count = 1  
        count = 1
        last_peak_idx = 0
        last_peak_val = 1
        # count of the last visited point
  # count at last peak
        for i in range(1, l):
            if ratings[i] > ratings[i - 1]:
                last_count += 1
                count += (last_count)
                last_peak_idx = i
                last_peak_val = last_count
            elif ratings[i] == ratings[i -1]:
                count = count + 1
                last_peak_idx = i
                last_peak_val = 1
                last_count = 1
            else: # rating less than prev
                count = count + 1
                last_count = 1
                if i - last_peak_idx < last_peak_val:
                    count += (i - last_peak_idx - 1)
                else: 
                    # peak inclusion
                    count += (i - last_peak_idx)

        return count
    

#print(Solution().candy([1,2,3,1,0]))
#                        2 1 2
print(Solution().candy([1,0,2]))