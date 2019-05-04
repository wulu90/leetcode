pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (0, 0);
        while i < nums.len() {
            if nums[i] != 0 {
                nums.swap(i, j);
                j += 1;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);

        let mut nums = vec![3, 1, 5, 3, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![3, 1, 5, 3, 0], nums);

        let mut nums = vec![0, 0, 5, 3, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![5, 3, 0, 0, 0], nums);
    }
}
