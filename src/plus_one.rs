pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut i: i32 = (digits.len() - 1) as i32;
        let mut add = 1;
        while i >= 0 {
            let sum = digits[i as usize] + add;
            result.insert(0, sum % 10);
            add = sum / 10;
            i = i - 1;
        }
        if add == 1 {
            result.insert(0, 1);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn plus_one() {
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
        assert_eq!(vec![1, 0, 0], Solution::plus_one(vec![9, 9]));
        assert_eq!(vec![1, 4], Solution::plus_one(vec![1, 3]));
    }
}
