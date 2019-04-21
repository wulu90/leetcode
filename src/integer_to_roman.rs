pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let number = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let ch = vec![
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];
        let mut s = String::from("");
        for i in 0..number.len() {
            let n = num / number[i];
            s.push_str(&ch[i].repeat(n as usize));
            num -= n * number[i];
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn int_to_roman() {
        assert_eq!(String::from("I"), Solution::int_to_roman(1));
        assert_eq!(String::from("III"), Solution::int_to_roman(3));
        assert_eq!(String::from("IV"), Solution::int_to_roman(4));
        assert_eq!(String::from("X"), Solution::int_to_roman(10));
        assert_eq!(String::from("IX"), Solution::int_to_roman(9));
        assert_eq!(String::from("XV"), Solution::int_to_roman(15));
        assert_eq!(String::from("XVIII"), Solution::int_to_roman(18));

        assert_eq!(String::from("XIX"), Solution::int_to_roman(19));
        assert_eq!(String::from("XXX"), Solution::int_to_roman(30));
        assert_eq!(String::from("LXXX"), Solution::int_to_roman(80));
        assert_eq!(String::from("XCIX"), Solution::int_to_roman(99));
        assert_eq!(String::from("CII"), Solution::int_to_roman(102));
        assert_eq!(String::from("CCC"), Solution::int_to_roman(300));
        assert_eq!(String::from("CD"), Solution::int_to_roman(400));
        assert_eq!(String::from("DCCC"), Solution::int_to_roman(800));
        assert_eq!(String::from("MCDXXXVII"), Solution::int_to_roman(1437));
        assert_eq!(String::from("MMMCCCXXXIII"), Solution::int_to_roman(3333));
        assert_eq!(String::from("MMMCMXCIX"), Solution::int_to_roman(3999));
    }
}
