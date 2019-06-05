/**
 * 49. Group Anagrams
 *
 * Given an array of strings, group anagrams together.
 *
 * Example:
 * Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
 * Output:
 * [
 *   ["ate","eat","tea"],
 *   ["nat","tan"],
 *   ["bat"]
 * ]
 *
 * Note:
 * All inputs will be in lowercase.
 * The order of your output does not matter.
 *
 */

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;
pub struct Solution;

impl Solution {
    pub fn group_anagrams_1(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return Vec::new();
        }
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            let t1 = Instant::now();

            let mut v = vec![0; 26];
            for c in s.bytes() {
                v[(c - 97) as usize] += 1;
            }
            let t2 = Instant::now();
            println!("t2-t1 {:?}", t2.duration_since(t1));

            let mut map_str = String::new();
            map_str.push('#');
            for count in v {
                map_str.push_str(&format!("{}{}", count, '#'));
                //map_str.push_str(&count.to_string());
                // map_str.push('#');
            }
            let t3 = Instant::now();
            println!("t3-t2 {:?}", t3.duration_since(t2));

            let va = map.entry(map_str).or_insert(vec![]);
            va.push(s);

            let t4 = Instant::now();
            println!("t4-t3 {:?}", t4.duration_since(t3));

        }
        map.values().map(|x| x.clone()).collect()
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return Vec::new();
        }
        let mut map: BTreeMap<Vec<u8>, Vec<String>> = BTreeMap::new();
        for s in strs {
            let mut sb = s.as_bytes().to_owned();
            sb.sort();

            let va = map.entry(sb).or_insert(vec![]);
            va.push(s);
        }
        //map.values().map(|x| x.clone()).collect()
        map.into_iter().map(|(_, x)| x).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn group_anagrams() {
        assert_eq!(
            vec![vec!["ate", "eat", "tea"], vec!["nat", "tan"], vec!["bat"]],
            Solution::group_anagrams(
                "eat,tea,tan,ate,nat,bat"
                    .split(',')
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>()
            )
        );
    }
}