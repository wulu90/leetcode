/**
 * 63. Unique Paths II
 *
 * A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
 *
 * The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
 *
 * Now consider if some obstacles are added to the grids. How many unique paths would there be?
 *
 * |------|------|------|------|------|------|------|
 * | start|      |      |      |      |      |      |
 * |------|------|------|------|------|------|------|
 * |      |  ob  |      |      |  ob  |      |      |
 * |------|------|------|------|------|------|------|
 * |      |      |      |      |      |      |finish|
 * |------|------|------|------|------|------|------|
 *
 * An obstacle and empty space is marked as 1 and 0 respectively in the grid.
 *
 * Note: m and n will be at most 100.
 *
 * Example 1:
 * Input:
 * [
 *   [0,0,0],
 *   [0,1,0],
 *   [0,0,0]
 * ]
 * Output: 2
 * Explanation:
 * There is one obstacle in the middle of the 3x3 grid above.
 * There are two ways to reach the bottom-right corner:
 * 1. Right -> Right -> Down -> Down
 * 2. Down -> Down -> Right -> Right
 *
 */

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        // let mut dp = obstacle_grid.clone();
        // this above will use more memory in leetcode
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            if obstacle_grid[i][0] == 0 {
                dp[i][0] = 1;
            } else if obstacle_grid[i][0] == 1 {
                for k in i..m {
                    dp[k][0] = 0;
                }
                break;
            }
        }

        // dp[0][0] can only be processed once
        for j in 1..n {
            if obstacle_grid[0][j] == 0 {
                dp[0][j] = 1;
            } else {
                for k in j..n {
                    dp[0][k] = 0;
                }
                break;
            }
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn unique_paths_with_obstacles() {
        assert_eq!(
            2,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
        assert_eq!(
            7,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0]
            ])
        );

        assert_eq!(
            4,
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 1, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0]
            ])
        );

        assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0]]));

        assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![1, 0]]));

        assert_eq!(
            0,
            Solution::unique_paths_with_obstacles(vec![vec![1], vec![0]])
        );
    }
}