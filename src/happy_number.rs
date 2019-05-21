use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        Self::cal_happy(n, &mut set)
    }

    fn cal_happy(mut n: i32, set: &mut HashSet<i32>) -> bool {
        if set.contains(&n) {
            return false;
        }
        set.insert(n);
        let mut sum = 0;
        while n > 0 {
            let i = n % 10;
            sum += i * i;
            n /= 10;
        }
        if sum == 1 {
            return true;
        } else {
            Self::cal_happy(sum, set)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn is_happy() {
        assert_eq!(true, Solution::is_happy(19));
    }
}
