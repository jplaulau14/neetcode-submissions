impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut curr = 0;
        for money in nums {
            let next = curr.max(prev + money);
            prev = curr;
            curr = next;
        }
        curr
    }
}