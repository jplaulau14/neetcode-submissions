from typing import List
from random import randrange

class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        def distance(p):
            return p[0] * p[0] + p[1] * p[1]

        lo, hi = 0, len(points) - 1
        target = k - 1
        while lo <= hi:
            pivot = distance(points[randrange(lo, hi + 1)])
            lt, i, gt = lo, lo, hi
            while i <= gt:
                d = distance(points[i])
                if d < pivot:
                    points[lt], points[i] = points[i], points[lt]
                    lt += 1
                    i += 1
                elif d > pivot:
                    points[i], points[gt] = points[gt], points[i]
                    gt -= 1
                else:
                    i += 1
            if target < lt:
                hi = lt - 1
            elif target > gt:
                lo = gt + 1
            else:
                break
        return points[:k]