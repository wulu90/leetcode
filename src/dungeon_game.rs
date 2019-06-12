/**
 * 174. Dungeon Game
 *
 * The demons had captured the princess (P) and imprisoned her in the bottom-right corner of a dungeon.
 * The dungeon consists of M x N rooms laid out in a 2D grid.
 * Our valiant knight (K) was initially positioned in the top-left room and must fight his way through the dungeon to rescue the princess.
 *
 * The knight has an initial health point represented by a positive integer.
 * If at any point his health point drops to 0 or below, he dies immediately.
 *
 * Some of the rooms are guarded by demons, so the knight loses health (negative integers) upon entering these rooms;
 * other rooms are either empty (0's) or contain magic orbs that increase the knight's health (positive integers).
 *
 * In order to reach the princess as quickly as possible, the knight decides to move only rightward or downward in each step.
 *
 * Write a function to determine the knight's minimum initial health so that he is able to rescue the princess.
 *
 * For example, given the dungeon below, the initial health of the knight must be at least 7
 * if he follows the optimal path RIGHT-> RIGHT -> DOWN -> DOWN.
 *
 * |------|------|------|
 * | -2(K)|  -3  |  3   |
 * |------|------|------|
 * |  -5  | -10  |  1   |
 * |------|------|------|
 * |  10  |  30  | -5(p)|
 * |------|------|------|
 *
 * Note:
 * The knight's health has no upper bound.
 * Any room can contain threats or power-ups, even the first room the knight enters and the bottom-right room where the princess is imprisoned.
 *
 */

pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.is_empty() || dungeon[0].is_empty() {
            return 0;
        }
        let row = dungeon.len();
        let col = dungeon[0].len();
        let mut dp = vec![vec![0_i32; col]; row];

        for i in (0..row).rev() {
            for j in (0..col).rev() {
                if i == row - 1 && j == col - 1 {
                    dp[i][j] = max(1, 1 - dungeon[i][j]);
                } else if i == row - 1 {
                    dp[i][j] = max(1, dp[i][j + 1] - dungeon[i][j]);
                } else if j == col - 1 {
                    dp[i][j] = max(1, dp[i + 1][j] - dungeon[i][j]);
                } else {
                    dp[i][j] = max(1, min(dp[i + 1][j], dp[i][j + 1]) - dungeon[i][j]);
                }
            }
        }
        dp[0][0]
    }

}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn calculate_minimum_hp() {
        assert_eq!(
            7,
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ])
        );

        assert_eq!(
            1,
            Solution::calculate_minimum_hp(vec![vec![3, -20, 30], vec![-3, 4, 0]])
        );

        assert_eq!(
            3,
            Solution::calculate_minimum_hp(vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]])
        );
    }
}