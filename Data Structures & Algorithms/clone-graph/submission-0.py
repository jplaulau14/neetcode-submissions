from typing import List, Optional


class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Optional['Node']) -> Optional['Node']:
        if node is None:
            return None
        clones = [None] * 101
        clones[node.val] = Node(node.val)
        stack = [node]
        while stack:
            current = stack.pop()
            copy = clones[current.val]
            for neighbor in current.neighbors:
                if clones[neighbor.val] is None:
                    clones[neighbor.val] = Node(neighbor.val)
                    stack.append(neighbor)
                copy.neighbors.append(clones[neighbor.val])
        return clones[node.val]