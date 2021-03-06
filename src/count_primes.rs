/**
 * 204 count primes
 *
 * Count the number of prime numbers less than a non-negative number, n.
 *
 * Example:
 * Input: 10
 * Output: 4
 * Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.
 */

pub struct Solution;

impl Solution {
    pub fn count_primes_1(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        let mut count = n - 1;
        for i in 2..n {
            for j in 2..=(i as f32).sqrt() as i32 + 1 {
                if i % j == 0 {
                    count -= 1;
                    break;
                }
            }
        }
        count
    }

    pub fn count_primes(n: i32) -> i32 {
        if n < 3 {
            return 0;
        }
        // 1,3,5,7,9,11,13,15,17,19
        let mut v = vec![true; n as usize / 2];
        let mut i = 1;
        let mut j;

        while i < (n as f32).sqrt() as i32 {
            j = i + 2 * i + 1;
            while j < n / 2 {
                v[j as usize] = false;
                j += 2 * i + 1;
            }
            i += 1;
        }

        let mut count = 1;
        for i in 1..n / 2 {
            if v[i as usize] {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn count_primes() {
        assert_eq!(0, Solution::count_primes(0));
        assert_eq!(0, Solution::count_primes(1));
        assert_eq!(0, Solution::count_primes(2));
        assert_eq!(1, Solution::count_primes(3));
        assert_eq!(2, Solution::count_primes(4));
        assert_eq!(2, Solution::count_primes(5));
        assert_eq!(3, Solution::count_primes(6));
        assert_eq!(3, Solution::count_primes(7));
        assert_eq!(4, Solution::count_primes(8));
        assert_eq!(4, Solution::count_primes(9));
        assert_eq!(4, Solution::count_primes(10));
        assert_eq!(25, Solution::count_primes(100));
        assert_eq!(168, Solution::count_primes(1000));
        assert_eq!(1229, Solution::count_primes(10000));
        //assert_eq!(105097564, Solution::count_primes(2147483647));
    }
}
