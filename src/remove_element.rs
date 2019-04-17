pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i = i + 1;
            }
            j = j + 1;
        }
        i as i32
    }

    pub fn remove_element1(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = nums.len();
        while i < j {
            if nums[i] == val {
                nums[i] = nums[j - 1];
                j = j - 1;
            } else {
                i = i + 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn remove_elementsss() {
        assert_eq!(2, Solution::remove_element(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(2, Solution::remove_element(&mut vec![3, 2, 2, 3], 2));
        assert_eq!(
            5,
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
        assert_eq!(0, Solution::remove_element(&mut vec![1], 1));
        assert_eq!(1, Solution::remove_element(&mut vec![1], 2));
        assert_eq!(0, Solution::remove_element(&mut vec![], 2));
    }

    #[test]
    fn remove_element1() {
        assert_eq!(2, Solution::remove_element1(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(2, Solution::remove_element1(&mut vec![3, 2, 2, 3], 2));
        assert_eq!(
            5,
            Solution::remove_element1(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
        assert_eq!(0, Solution::remove_element1(&mut vec![1], 1));
        assert_eq!(1, Solution::remove_element1(&mut vec![1], 2));
        assert_eq!(0, Solution::remove_element1(&mut vec![], 2));
    }
}
