impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut size = vec![1usize; n + 1];
        fn find(parent: &mut [usize], mut node: usize) -> usize {
            while parent[node] != node {
                parent[node] = parent[parent[node]];
                node = parent[node];
            }
            node
        }
        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            let mut root_a = find(&mut parent, a);
            let mut root_b = find(&mut parent, b);
            if root_a == root_b {
                return vec![edge[0], edge[1]];
            }
            if size[root_a] < size[root_b] {
                std::mem::swap(&mut root_a, &mut root_b);
            }
            parent[root_b] = root_a;
            size[root_a] += size[root_b];
        }
        vec![edges[n - 1][0], edges[n - 1][1]]
    }
}