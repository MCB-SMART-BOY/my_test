use std::collections::VecDeque;

pub struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut result = Vec::with_capacity(n - k + 1);
        let mut deque = VecDeque::new();

        for i in 0..n {
            while let Some(&back_idx) = deque.back() {
                if nums[back_idx] <= nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);

            if i >= k - 1 {
                while let Some(&front_idx) = deque.front() {
                    if front_idx < i + 1 - k {
                        deque.pop_front();
                    } else {
                        break;
                    }
                }
                result.push(nums[deque[0]]);
            }
        }

        result
    }
}

pub fn test() {
    let input = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let result = Solution::max_sliding_window(input, k);
    println!("题目结果：{result:?}");
}
