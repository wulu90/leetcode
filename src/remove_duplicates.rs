pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut i = 0;
        let mut j = 1;
        while j < nums.len() {
            if nums[j] != nums[i] {
                i = i + 1;
                nums[i] = nums[j];
            }
            j = j + 1;
        }
        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;
    #[test]
    fn remove_duplicates() {
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
        assert_eq!(
            5,
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
        assert_eq!(1, Solution::remove_duplicates(&mut vec![1]));
        assert_eq!(0, Solution::remove_duplicates(&mut vec![]));
    }
}
