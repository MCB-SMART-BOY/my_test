pub struct Solution;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            while nums[i] >= 1 && nums[i] <= (n as i32) && nums[(nums[i] - 1) as usize] != nums[i] {
                let idx = (nums[i] - 1) as usize;
                nums.swap(i, idx);
            }
        }

        for (i, &x) in nums.iter().enumerate() {
            if x != (i as i32) + 1 {
                return (i as i32) + 1;
            }
        }

        n as i32 + 1
    }
}

pub fn test() {
    let input = vec![1, 2, 0];
    let result = Solution::first_missing_positive(input);
    println!("题目结果：{result:?}");
}
