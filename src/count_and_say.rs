pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = String::new();
        for i in 1..n + 1 {
            if i == 1 {
                s = String::from("1");
            } else if i == 2 {
                s = String::from("11");
            } else {
                let mut times = 1;
                let mut temp = String::from("");
                let mut pre_str = &s[0..1];
                for j in 1..s.len() {
                    if &s[j..j + 1] != pre_str {
                        temp.push(std::char::from_digit(times, 10).unwrap());
                        temp.push_str(pre_str);
                        times = 1;
                        pre_str = &s[j..j + 1]
                    } else {
                        times += 1;
                    }

                    if j == s.len() - 1 {
                        temp.push(std::char::from_digit(times, 10).unwrap());
                        temp.push_str(&s[j..j + 1]);
                    }
                }
                s = temp;
            }
        }
        s
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn count_and_say() {
        assert_eq!("1", Solution::count_and_say(1));
        assert_eq!("11", Solution::count_and_say(2));
        assert_eq!("21", Solution::count_and_say(3));
        assert_eq!("1211", Solution::count_and_say(4));
        assert_eq!("111221", Solution::count_and_say(5));
        assert_eq!("312211", Solution::count_and_say(6));
        assert_eq!("13112221", Solution::count_and_say(7));
        assert_eq!("1113213211", Solution::count_and_say(8));
        assert_eq!("31131211131221", Solution::count_and_say(9));
        assert_eq!("13211311123113112211", Solution::count_and_say(10));
        assert_eq!("11131221133112132113212221", Solution::count_and_say(11));
        assert_eq!(
            "3113112221232112111312211312113211",
            Solution::count_and_say(12)
        );
        assert_eq!(
            "1321132132111213122112311311222113111221131221",
            Solution::count_and_say(13)
        );
        assert_eq!(
            "11131221131211131231121113112221121321132132211331222113112211",
            Solution::count_and_say(14)
        );
    }
}
