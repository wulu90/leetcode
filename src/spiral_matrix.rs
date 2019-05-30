/**
 * 54. Spiral Matrix
 *
 * Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
 *
 * Example 1:
 * Input:
 * [
 *  [ 1, 2, 3 ],
 *  [ 4, 5, 6 ],
 *  [ 7, 8, 9 ]
 * ]
 * Output: [1,2,3,6,9,8,7,4,5]
 *
 * Example 2:
 * Input:
 * [
 *   [1, 2, 3, 4],
 *   [5, 6, 7, 8],
 *   [9,10,11,12]
 * ]
 * Output: [1,2,3,4,8,12,11,10,9,5,6,7]
 */

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut v: Vec<i32> = Vec::with_capacity(m * n);

        let (mut c1, mut c2, mut r1, mut r2) = (0, n - 1, 0, m - 1);
        while c1 <= c2 && r1 <= r2 {
            for i in c1..=c2 {
                v.push(matrix[r1][i]);
            }
            r1 += 1;
            if r1 > r2 {
                break;
            }
            for i in r1..=r2 {
                v.push(matrix[i][c2]);
            }
            //c2 -= 1; c2 is usize,may overflow, can't use (c2,overflow),shadow dosn't change orignal value;
            let (c, overflow) = c2.overflowing_sub(1);
            if overflow || c < c1 {
                break;
            }
            c2 = c;
            for i in (c1..=c2).rev() {
                v.push(matrix[r2][i]);
            }
            //r2 -= 1;
            let (r, overflow) = r2.overflowing_sub(1);
            if overflow || r2 < r1 {
                break;
            }
            r2 = r;
            for i in (r1..=r2).rev() {
                v.push(matrix[i][c1]);
            }
            if c1 > c2 {
                break;
            }
            c1 += 1;
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn spiral_order() {
        assert_eq!(vec![] as Vec<i32>, Solution::spiral_order(vec![]));
        assert_eq!(vec![] as Vec<i32>, Solution::spiral_order(vec![vec![]]));
        assert_eq!(vec![3, 2], Solution::spiral_order(vec![vec![3], vec![2]]));
        assert_eq!(
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ])
        );

        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::spiral_order(vec![vec![1, 2, 3, 4]])
        );

        assert_eq!(
            vec![1, 2, 4, 3],
            Solution::spiral_order(vec![vec![1, 2], vec![3, 4]])
        );

        assert_eq!(
            vec![1, 2, 3, 4, 13, 14, 16, 18, 17, 12, 11, 10, 9, 5, 6, 7, 8, 15],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4, 13, 14],
                vec![5, 6, 7, 8, 15, 16],
                vec![9, 10, 11, 12, 17, 18]
            ])
        );

        assert_eq!(
            vec![
                1, 2, 3, 4, 8, 12, 16, 20, 24, 28, 27, 26, 25, 21, 17, 13, 9, 5, 6, 7, 11, 15, 19,
                23, 22, 18, 14, 10
            ],
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
                vec![17, 18, 19, 20],
                vec![21, 22, 23, 24],
                vec![25, 26, 27, 28],
            ])
        );
    }
}