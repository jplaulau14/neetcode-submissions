from collections import deque


class Solution:
    def findOrder(self, numCourses: int, prerequisites: list[list[int]]) -> list[int]:
        outgoing = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for course, prerequisite in prerequisites:
            outgoing[prerequisite].append(course)
            indegree[course] += 1
        ready = deque(course for course in range(numCourses) if indegree[course] == 0)
        order = []
        while ready:
            course = ready.popleft()
            order.append(course)
            for dependent in outgoing[course]:
                indegree[dependent] -= 1
                if indegree[dependent] == 0:
                    ready.append(dependent)
        return order if len(order) == numCourses else []