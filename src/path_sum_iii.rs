use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn path_sum_iii(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut s = 0;
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            s += Self::dfs(node.clone(), sum, 0);

            if node.as_ref().unwrap().borrow().left.is_some() {
                queue.push_back(node.as_ref().unwrap().borrow().left.clone());
            }
            if node.as_ref().unwrap().borrow().right.is_some() {
                queue.push_back(node.as_ref().unwrap().borrow().right.clone());
            }
        }
        s
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, sum: i32, cur: i32) -> i32 {
        let mut a = 0;
        if let Some(v) = node {
            if v.borrow().val + cur == sum {
                a += 1;
            }
            if v.borrow().left.is_some() {
                a += Self::dfs(v.borrow().left.clone(), sum, v.borrow().val + cur);
            }
            if v.borrow().right.is_some() {
                a += Self::dfs(v.borrow().right.clone(), sum, v.borrow().val + cur);
            }
        }
        a
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn path_sum_iii() {
        assert_eq!(
            3,
            Solution::path_sum_iii(binarytree![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1], 8)
        );
    }
}
