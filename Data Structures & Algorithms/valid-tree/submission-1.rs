use std::collections::VecDeque;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        if edges.len() != n - 1 {
            return false;
        }
        let mut adj = vec![Vec::new(); n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            adj[a].push(b);
            adj[b].push(a);
        }
        let mut seen = vec![false; n];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        seen[0] = true;
        let mut visited = 0usize;
        while let Some(node) = queue.pop_front() {
            visited += 1;
            for &neighbor in &adj[node] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
        visited == n
    }
}