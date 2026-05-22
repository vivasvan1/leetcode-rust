from typing import List

def threeSum(nums: List[int]) -> List[List[int]]:
    lists = []

    nums = sorted(nums)

    hashmap = {}
    for i,v in enumerate(nums):
        hashmap.update({v:i})
    
    print(nums, hashmap)

    i = 0
    j = i+1

    while i <len(nums)-2:
        
        if(nums[i]>0):
            break
        j = i+1
        while j < len(nums)-1:
            k_needed = -(nums[i] + nums[j])
            if (k_needed in hashmap.keys()) and (hashmap[k_needed] > j):
                lists.append([nums[i],nums[j],k_needed])
            j = hashmap[nums[j]]
            j+=1
        i=hashmap[nums[i]]
        i+=1
    return lists

threeSum([-1,0,1,2,-1,-4])
