use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut pmap = Self::map(&p);
        let mut v = Vec::new();

        let mut start = 0;
        let mut end = 0;
        let mut unique_count = pmap.len();
        let chars: Vec<char> = s.chars().collect();
        while start < s.len() && end < s.len() {
            if pmap.contains_key(&chars[end]) {
                pmap.insert(chars[end], *pmap.get(&chars[end]).unwrap() - 1);
                if pmap.get(&chars[end]) == Some(&0) {
                    unique_count -= 1;
                }
            }
            if unique_count == 0 {
                v.push(start as i32);
            }
            end += 1;
            if end - start + 1 > p.len() || !pmap.contains_key(&chars[start]) {
                if pmap.contains_key(&chars[start]) {
                    if pmap.get(&chars[start]) == Some(&0) {
                        unique_count += 1;
                    }
                    pmap.insert(chars[start], *pmap.get(&chars[start]).unwrap() + 1);
                }
                start += 1;
            }
        }
        v
    }

    fn map(s: &str) -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for i in s.chars() {
            map.insert(i, *map.get(&i).unwrap_or(&0) + 1);
        }
        map
    }

    //https://leetcode.com/submissions/detail/226662040/ time limit
    pub fn find_anagrams_tle(s: String, p: String) -> Vec<i32> {
        let pmap = Self::map(&p);
        let mut v = Vec::new();
        if s.len() > p.len() {
            for i in 0..s.len() - p.len() + 1 {
                if pmap == Self::map(&s[i..i + p.len()]) {
                    v.push(i as i32);
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn find_anagrams() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("abc"))
        );

        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams(String::from("abab"), String::from("ab"))
        );

        assert_eq!(
            vec![1, 3],
            Solution::find_anagrams(String::from("cbaebabacd"), String::from("baeb"))
        );

        assert_eq!(
            vec![] as Vec<i32>,
            Solution::find_anagrams(String::from(""), String::from("baeb"))
        );
    }
}
