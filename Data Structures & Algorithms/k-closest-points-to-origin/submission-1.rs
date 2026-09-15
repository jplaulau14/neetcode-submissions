use std::collections::BinaryHeap;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for point in points {
            let (x, y) = (point[0], point[1]);
            let d = i64::from(x) * i64::from(x) + i64::from(y) * i64::from(y);
            heap.push((d, x, y));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, x, y)| vec![x, y]).collect()
    }
}