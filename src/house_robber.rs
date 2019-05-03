pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() < 2 {
            return nums[0];
        }
        let mut arr = Vec::with_capacity(nums.len());
        arr.push(nums[0]);
        arr.push(std::cmp::max(nums[0], nums[1]));
        for i in 2..nums.len() {
            arr.push(std::cmp::max(nums[i] + arr[i - 2], arr[i - 1]));
        }
        arr[nums.len() - 1]
    }

    pub fn rob1(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() < 2 {
            return nums[0];
        }
        let mut pre = 0;
        let mut max = nums[0];
        for i in 1..nums.len() {
            let tmp = max;
            max = std::cmp::max(pre + nums[i], max);
            pre = tmp;
        }
        max
    }

    /**
     * max = Self::robr(&nums, nums.len() as i32 - 1);
     * Time Limit Exceeded
     */
    pub fn robr(nums: &Vec<i32>, n: i32) -> i32 {
        if n < 0 {
            return 0;
        }
        return std::cmp::max(
            Self::robr(nums, n - 1),
            Self::robr(nums, n - 2) + nums[n as usize],
        );
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn rob() {
        assert_eq!(1, Solution::rob1(vec![1]));
        assert_eq!(2, Solution::rob1(vec![1, 2]));
        assert_eq!(4, Solution::rob1(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob1(vec![2, 7, 9, 3, 1]));
        assert_eq!(
            4173,
            Solution::rob(vec![
                114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170,
                110, 236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64,
                214, 228, 78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240
            ])
        )
    }
}
