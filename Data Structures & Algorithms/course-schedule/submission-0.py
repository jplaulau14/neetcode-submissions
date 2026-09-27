from collections import deque


class Solution:
    def canFinish(self, numCourses: int, prerequisites: list[list[int]]) -> bool:
        outgoing = [[] for _ in range(numCourses)]
        indegree = [0] * numCourses
        for course, prerequisite in prerequisites:
            outgoing[prerequisite].append(course)
            indegree[course] += 1
        ready = deque(course for course in range(numCourses) if indegree[course] == 0)
        completed = 0
        while ready:
            course = ready.popleft()
            completed += 1
            for dependent in outgoing[course]:
                indegree[dependent] -= 1
                if indegree[dependent] == 0:
                    ready.append(dependent)
        return completed == numCourses