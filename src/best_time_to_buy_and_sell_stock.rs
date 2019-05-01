pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..prices.len() {
            for j in i..prices.len() {
                if prices[j] > prices[i] {
                    max = max.max(prices[j] - prices[i]);
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn max_profit() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 5, 4, 3]));
    }
}
