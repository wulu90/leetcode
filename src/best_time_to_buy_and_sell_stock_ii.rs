pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::max_value();
        let mut profit = 0;
        for i in 0..prices.len() {
            if prices[i] < min_price {
                min_price = prices[i];
            } else {
                profit += prices[i] - min_price;
                min_price = prices[i];
            }
        }
        profit
    }
}

#[cfg(test)]
mod test {

    use super::Solution;

    #[test]
    fn max_profit() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(0, Solution::max_profit(vec![]));
    }
}
