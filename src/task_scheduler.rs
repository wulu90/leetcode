/**
 * 621. Task Scheduler
 *
 * Given a char array representing tasks CPU need to do.
 * It contains capital letters A to Z where different letters represent different tasks.
 * Tasks could be done without original order. Each task could be done in one interval.
 * For each interval, CPU could finish one task or just be idle.
 *
 * However, there is a non-negative cooling interval n that means between two same tasks,
 * there must be at least n intervals that CPU are doing different tasks or just be idle.
 *
 * You need to return the least number of intervals the CPU will take to finish all the given tasks.
 *
 * Example:
 * Input: tasks = ["A","A","A","B","B","B"], n = 2
 * Output: 8
 * Explanation: A -> B -> idle -> A -> B -> idle -> A -> B.
 *
 * Note:
 * The number of tasks is in the range [1, 10000].
 * The integer n is in the range [0, 100].
 *
 */

pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = vec![0; 26];
        for c in tasks {
            map[(c as u8 - 65) as usize] += 1;
        }
        map.sort();
        let mut time = 0;
        let n: usize = n as usize;
        while map[25] > 0 {
            let mut i: usize = 0;
            while i <= n {
                if map[25] == 0 {
                    break;
                }
                if i < 26 && map[25 - i] > 0 {
                    map[25 - i] -= 1;
                }
                time += 1;
                i += 1;
            }
            map.sort();
        }

        time
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn least_interval() {
        assert_eq!(
            8,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2)
        );

        assert_eq!(
            10,
            Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 3)
        );
    }
}