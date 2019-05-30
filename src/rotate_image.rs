/**
 * 48. Rotate Image
 *
 * You are given an n x n 2D matrix representing an image.
 * Rotate the image by 90 degrees (clockwise).
 *
 * Note:
 * You have to rotate the image in-place, which means you have to modify the input 2D matrix directly.
 * DO NOT allocate another 2D matrix and do the rotation.
 *
 * Example 1:
 * Given input matrix =
 * [
 *   [1,2,3],
 *   [4,5,6],
 *   [7,8,9]
 * ],
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [7,4,1],
 *   [8,5,2],
 *   [9,6,3]
 * ]
 *
 * Example 2:
 * Given input matrix =
 * [
 *   [ 5, 1, 9,11],
 *   [ 2, 4, 8,10],
 *   [13, 3, 6, 7],
 *   [15,14,12,16]
 * ],
 * rotate the input matrix in-place such that it becomes:
 * [
 *   [15,13, 2, 5],
 *   [14, 3, 4, 1],
 *   [12, 6, 8, 9],
 *   [16, 7,10,11]
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn rotate_image(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            for j in i..n - 1 - i {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[n - 1 - j][i];
                matrix[n - 1 - j][i] = matrix[n - 1 - i][n - 1 - j];
                matrix[n - 1 - i][n - 1 - j] = matrix[j][n - 1 - i];
                matrix[j][n - 1 - i] = temp;
            }
        }
    }

    pub fn rotate_1(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        //swap the right top triangle and the left bottom triangle by the diagonal from left top to right bottom
        for i in 0..n {
            for j in i..n {
                if i != j {
                    matrix[i][j] = matrix[i][j] ^ matrix[j][i];
                    matrix[j][i] = matrix[j][i] ^ matrix[i][j];
                    matrix[i][j] = matrix[i][j] ^ matrix[j][i];
                }
            }
        }
        // swap by column from left to right
        for i in 0..n {
            for j in 0..n / 2 {
                matrix[i][j] = matrix[i][j] ^ matrix[i][n - 1 - j];
                matrix[i][n - 1 - j] = matrix[i][n - 1 - j] ^ matrix[i][j];
                matrix[i][j] = matrix[i][j] ^ matrix[i][n - 1 - j];
            }
        }
        // another way is:
        // swap the left top triangle and the right bottom triangle by the diagonal from right top to left bottom
        // then swap by row from top to bottom
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn rotate_image() {
        let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate_image(&mut matrix);
        assert_eq!(vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]], matrix);
        matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate_image(&mut matrix);
        assert_eq!(
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ],
            matrix
        );
    }
}