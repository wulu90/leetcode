/**
 * 73. Set Matrix Zeroes
 *
 * Given a m x n matrix, if an element is 0, set its entire row and column to 0. Do it in-place.
 *
 * Example 1:
 * Input:
 * [
 *   [1,1,1],
 *   [1,0,1],
 *   [1,1,1]
 * ]
 * Output:
 * [
 *   [1,0,1],
 *   [0,0,0],
 *   [1,0,1]
 * ]
 *
 * Example 2:
 * Input:
 * [
 *   [0,1,2,0],
 *   [3,4,5,2],
 *   [1,3,1,5]
 * ]
 * Output:
 * [
 *   [0,0,0,0],
 *   [0,4,5,0],
 *   [0,3,1,0]
 * ]
 *
 * Follow up:
 * A straight forward solution using O(mn) space is probably a bad idea.
 * A simple improvement uses O(m + n) space, but still not the best solution.
 * Could you devise a constant space solution?
 *
 */

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        let row_n = matrix.len();
        let col_n = matrix[0].len();
        let mut row = vec![false; row_n];
        let mut col = vec![false; col_n];
        for i in 0..row_n {
            for j in 0..col_n {
                if matrix[i][j] == 0 {
                    row[i] = true;
                    col[j] = true;
                }
            }
        }
        for i in 0..row_n {
            for j in 0..col_n {
                if row[i] || col[j] {
                    matrix[i][j] = 0;
                }
            }
        }

        /*
        for i in 0..row_n {
            if row[i] {
                for j in 0..col_n {
                    matrix[i][j] = 0;
                }
            }
        }

        for j in 0..col_n {
            if col[j] {
                for i in 0..row_n {
                    matrix[i][j] = 0;
                }
            }
        }
        */
    }

    pub fn set_zeroes_1(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }
        let mut col_1 = false;

        let m = matrix.len();
        let n = matrix[0].len();

        for i in 0..m {
            if matrix[i][0] == 0 {
                col_1 = true;
            }

            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for j in 0..n {
                matrix[0][j] = 0;
            }
        }

        if col_1 {
            for i in 0..m {
                matrix[i][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn set_zeroes() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], matrix);

        matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            matrix
        );

        matrix = vec![] as Vec<Vec<i32>>;
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![] as Vec<Vec<i32>>, matrix);

        matrix = vec![vec![]] as Vec<Vec<i32>>;
        Solution::set_zeroes(&mut matrix);
        assert_eq!(vec![vec![]] as Vec<Vec<i32>>, matrix);

    }
}