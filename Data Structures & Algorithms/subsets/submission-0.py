class Solution:
    def subsets(self, nums):
        result = []
        path = []

        def dfs(index):
            if index == len(nums):
                result.append(path.copy())
                return
            dfs(index + 1)
            path.append(nums[index])
            dfs(index + 1)
            path.pop()

        dfs(0)
        return result