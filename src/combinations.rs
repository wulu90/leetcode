/**
 * 77. Combinations
 *
 * Given two integers n and k, return all possible combinations of k numbers out of 1 ... n.
 *
 * Example:
 * Input: n = 4, k = 2
 * Output:
 * [
 *   [2,4],
 *   [3,4],
 *   [2,3],
 *   [1,2],
 *   [1,3],
 *   [1,4],
 * ]
 *
 */

pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::backtracking(n, k, 1, &mut Vec::new(), &mut res);
        res
    }

    fn backtracking(n: i32, k: i32, cur: i32, v: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if v.len() == k as usize {
            res.push(v.clone());
        } else {
            for i in cur..=n {
                v.push(i);
                Self::backtracking(n, k, i + 1, v, res);
                v.pop();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn combine() {
        assert_eq!(
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ],
            Solution::combine(4, 2)
        );

        assert_eq!(
            vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]],
            Solution::combine(4, 3)
        );

        assert_eq!(vec![] as Vec<Vec<i32>>, Solution::combine(0, 3));

        assert_eq!(vec![vec![]] as Vec<Vec<i32>>, Solution::combine(3, 0));
    }
}