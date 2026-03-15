use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = HashMap::with_capacity(nums.len());
        let mut current_sum = 0;
        let mut result = 0;

        for num in nums {
            *prefix.entry(current_sum).or_insert(0) += 1;
            current_sum += num;
            if let Some(&count) = prefix.get(&(current_sum - k)) {
                result += count;
            }
        }
        result
    }
}

pub fn test() {
    let input = vec![1, 2, 3];
    let k = 3;
    let result = Solution::subarray_sum(input, k);
    println!("题目结果：{result:?}");
}
