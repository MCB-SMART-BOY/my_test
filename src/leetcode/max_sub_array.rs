pub struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut window_sum = 0;
        let mut max_sum = i32::MIN;

        for num in nums {
            window_sum += num;
            max_sum = max_sum.max(window_sum);

            if window_sum < 0 {
                window_sum = 0;
            }
        }

        max_sum
    }
}

pub fn test() {
    let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let result = Solution::max_sub_array(input);
    println!("题目结果：{result:?}");
}
