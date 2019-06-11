/**
 * 62. Unique Paths
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 * How many possible unique paths are there?
 *
 * |------|------|------|------|------|------|------|
 * | start|      |      |      |      |      |      |
 * |------|------|------|------|------|------|------|
 * |      |      |      |      |      |      |      |
 * |------|------|------|------|------|------|------|
 * |      |      |      |      |      |      |finish|
 * |------|------|------|------|------|------|------|
 *
 * Above is a 7 x 3 grid. How many possible unique paths are there?
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 * Input: m = 3, n = 2
 * Output: 3
 * Explanation:
 * From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down
 * 2. Right -> Down -> Right
 * 3. Down -> Right -> Right
 *
 * Example 2:
 * Input: m = 7, n = 3
 * Output: 28
 *
 */

pub struct Solution;

impl Solution {

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut step = vec![vec![1_i32; n]; m];

        for i in 1..m {
            for j in 1..n {
                step[i][j] = step[i - 1][j] + step[i][j - 1];
            }
        }
        step[m - 1][n - 1]
    }


    pub fn unique_paths_recursive(m: i32, n: i32) -> i32 {
        let mut num = 0;
        Self::helper(m - 1, n - 1, 0, 0, &mut num);
        num
    }

    fn helper(m: i32, n: i32, right: i32, down: i32, num: &mut i32) {
        if right == m || down == n {
            *num += 1;
        } else {
            if right < m {
                Self::helper(m, n, right + 1, down, num);
            }
            if down < n {
                Self::helper(m, n, right, down + 1, num);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn unique_paths() {
        assert_eq!(1, Solution::unique_paths(3, 1));
        assert_eq!(1, Solution::unique_paths(1, 2));
        assert_eq!(3, Solution::unique_paths(3, 2));
        assert_eq!(28, Solution::unique_paths(7, 3));
        assert_eq!(155117520, Solution::unique_paths(16, 16));
        assert_eq!(1916797311, Solution::unique_paths(51, 9));
    }
}