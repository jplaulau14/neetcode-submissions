impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut counts = vec![0i32; 20001];
        for value in nums {
            counts[(value + 10000) as usize] += 1;
        }
        for index in (0..20001).rev() {
            k -= counts[index];
            if k <= 0 {
                return index as i32 - 10000;
            }
        }
        unreachable!()
    }
}