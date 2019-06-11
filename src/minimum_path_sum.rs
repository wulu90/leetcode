/**
 * 64. Minimum Path Sum
 *
 * Given a m x n grid filled with non-negative numbers,
 * find a path from top left to bottom right which minimizes the sum of all numbers along its path.
 *
 * Note: You can only move either down or right at any point in time.
 *
 * Example:
 * Input:
 * [
 *   [1,3,1],
 *   [1,5,1],
 *   [4,2,1]
 * ]
 * Output: 7
 * Explanation: Because the path 1→3→1→1→1 minimizes the sum.
 *
 */

pub struct Solution;

impl Solution {

    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = grid[0][0];

        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }

    // time limit
    pub fn min_path_sum_recursive(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let mut num = vec![];
        let m = grid.len();
        let n = grid[0].len();
        Self::helper(m - 1, n - 1, 0, 0, &grid, grid[0][0], &mut num);
        num.sort();
        num[0]
    }

    fn helper(
        m: usize,
        n: usize,
        right: usize,
        down: usize,
        grid: &Vec<Vec<i32>>,
        sum: i32,
        num: &mut Vec<i32>,
    ) {
        if right == n && down == m {
            num.push(sum);
        } else {
            if right < n {
                Self::helper(
                    m,
                    n,
                    right + 1,
                    down,
                    &grid,
                    sum + grid[down][right + 1],
                    num,
                );
            }
            if down < m {
                Self::helper(
                    m,
                    n,
                    right,
                    down + 1,
                    &grid,
                    sum + grid[down + 1][right],
                    num,
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn min_path_sum() {
        assert_eq!(
            8,
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 2]])
        );

        assert_eq!(0, Solution::min_path_sum(vec![vec![0]]));

        assert_eq!(5, Solution::min_path_sum(vec![vec![1, 3, 1]]));

        assert_eq!(6, Solution::min_path_sum(vec![vec![1], vec![1], vec![4]]));

        assert_eq!(
            83,
            Solution::min_path_sum(vec![
                vec![5, 0, 1, 1, 2, 1, 0, 1, 3, 6, 3, 0, 7, 3, 3, 3, 1],
                vec![1, 4, 1, 8, 5, 5, 5, 6, 8, 7, 0, 4, 3, 9, 9, 6, 0],
                vec![2, 8, 3, 3, 1, 6, 1, 4, 9, 0, 9, 2, 3, 3, 3, 8, 4],
                vec![3, 5, 1, 9, 3, 0, 8, 3, 4, 3, 4, 6, 9, 6, 8, 9, 9],
                vec![3, 0, 7, 4, 6, 6, 4, 6, 8, 8, 9, 3, 8, 3, 9, 3, 4],
                vec![8, 8, 6, 8, 3, 3, 1, 7, 9, 3, 3, 9, 2, 4, 3, 5, 1],
                vec![7, 1, 0, 4, 7, 8, 4, 6, 4, 2, 1, 3, 7, 8, 3, 5, 4],
                vec![3, 0, 9, 6, 7, 8, 9, 2, 0, 4, 6, 3, 9, 7, 2, 0, 7],
                vec![8, 0, 8, 2, 6, 4, 4, 0, 9, 3, 8, 4, 0, 4, 7, 0, 4],
                vec![3, 7, 4, 5, 9, 4, 9, 7, 9, 8, 7, 4, 0, 4, 2, 0, 4],
                vec![5, 9, 0, 1, 9, 1, 5, 9, 5, 5, 3, 4, 6, 9, 8, 5, 6],
                vec![5, 7, 2, 4, 4, 4, 2, 1, 8, 4, 8, 0, 5, 4, 7, 4, 7],
                vec![9, 5, 8, 6, 4, 4, 3, 9, 8, 1, 1, 8, 7, 7, 3, 6, 9],
                vec![7, 2, 3, 1, 6, 3, 6, 6, 6, 3, 2, 3, 9, 9, 4, 4, 8]
            ])
        )
    }
}