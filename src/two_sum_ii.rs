use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        while i < j {
            if numbers[i] + numbers[j] > target {
                j -= 1;
            } else if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                return vec![i as i32 + 1, j as i32 + 1];
            }
        }
        vec![]
    }

    pub fn two_sum_ii_1(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, numbers.len() - 1);
        loop {
            match target.cmp(&(numbers[i] + numbers[j])) {
                Ordering::Less => j -= 1,
                Ordering::Greater => i += 1,
                Ordering::Equal => return vec![i as i32 + 1, j as i32 + 1],
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn two_sum_ii() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));

        assert_eq!(vec![1, 2], Solution::two_sum_ii_1(vec![2, 7, 11, 15], 9));
    }
}
