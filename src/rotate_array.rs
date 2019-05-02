pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k: usize = k as usize % nums.len();
        let mut a: Vec<i32> = vec![0; nums.len()];
        for i in 0..nums.len() {
            a[(i + k) % nums.len()] = nums[i];
        }
        for i in 0..nums.len() {
            nums[i] = a[i];
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn rotate() {
        let mut vec = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut vec, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], vec);
    }
}
