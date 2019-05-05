pub struct Solution;

impl Solution {
    pub fn hammming_distance(x: i32, y: i32) -> i32 {
        //https://en.wikipedia.org/wiki/Hamming_distance
        let mut dist = 0;
        let mut v = x ^ y;
        while v > 0 {
            if v & 1 > 0 {
                dist += 1;
            }
            v /= 2;
        }
        dist
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn hammming_distance() {
        assert_eq!(2, Solution::hammming_distance(1, 4));
    }
}
