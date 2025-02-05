from typing import List


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prod = [1]
        if len(nums) == 1: 
            return prod
        
        for i in range(1, len(nums)):
            # calculate prod for the curr as pre number time product
            prod.append(nums[i-1] * prod[i-1])
            # prod num to all existing
            for j in range(0, i):
                prod[j] = prod[j] * nums[i]

        return prod
    

m = Solution()
ret = m.productExceptSelf([1,2,3,4])
print(ret)