impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..n).collect();
        let mut size = vec![1usize; n];
        fn find(parent: &mut [usize], mut node: usize) -> usize {
            while parent[node] != node {
                parent[node] = parent[parent[node]];
                node = parent[node];
            }
            node
        }
        let mut components = n;
        for edge in &edges {
            let mut root_a = find(&mut parent, edge[0] as usize);
            let mut root_b = find(&mut parent, edge[1] as usize);
            if root_a == root_b {
                continue;
            }
            if size[root_a] < size[root_b] {
                std::mem::swap(&mut root_a, &mut root_b);
            }
            parent[root_b] = root_a;
            size[root_a] += size[root_b];
            components -= 1;
        }
        components as i32
    }
}