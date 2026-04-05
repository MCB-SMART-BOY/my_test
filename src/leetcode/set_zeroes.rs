pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();

        // 单独记录第一行、第一列原本是否含 0
        let first_row_zero = matrix[0].contains(&0);
        let first_col_zero = matrix.iter().any(|row| row[0] == 0);

        // 用第一行、第一列做标记
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        // 根据标记，把内部区域置 0
        for i in 1..m {
            for j in 1..n {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        // 最后处理第一行
        if first_row_zero {
            for x in matrix[0].iter_mut() {
                *x = 0;
            }
        }

        // 最后处理第一列
        if first_col_zero {
            for y in matrix.iter_mut() {
                y[0] = 0;
            }
        }
    }
}

pub fn test() {
    let mut input = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut input);
    println!("题目结果：{:?}", input);
}
