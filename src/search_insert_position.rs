pub struct Solution;

impl Solution {
    pub fn serch_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] == target {
                return m as i32;
            } else if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn search_insert_position() {
        assert_eq!(2, Solution::serch_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::serch_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::serch_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution::serch_insert(vec![1, 3, 5, 6], 0));

        assert_eq!(0, Solution::serch_insert(vec![], 0));
    }
}
