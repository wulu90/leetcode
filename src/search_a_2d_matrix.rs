/**
 * 74. Search a 2D Matrix
 *
 * Write an efficient algorithm that searches for a value in an m x n matrix. This matrix has the following properties:
 *
 * Integers in each row are sorted from left to right.
 * The first integer of each row is greater than the last integer of the previous row.
 *
 * Example 1:
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 3
 * Output: true
 *
 * Example 2:
 * Input:
 * matrix = [
 *   [1,   3,  5,  7],
 *   [10, 11, 16, 20],
 *   [23, 30, 34, 50]
 * ]
 * target = 13
 * Output: false
 *
 */

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let row = matrix.len();
        let col = matrix[0].len();
        if target < matrix[0][0] || target > matrix[row - 1][col - 1] {
            return false;
        }

        let (mut l, mut r) = (0, row * col - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            if matrix[mid / col][mid % col] == target {
                return true;
            } else if matrix[mid / col][mid % col] > target {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn search_matrix() {
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                3
            )
        );
        assert_eq!(
            false,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                13
            )
        );
        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                1
            )
        );

        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                50
            )
        );

        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                11
            )
        );

        assert_eq!(
            true,
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
                16
            )
        );
    }
}