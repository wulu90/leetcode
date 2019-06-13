/**
 * 102. Binary Tree Level Order Traversal
 *
 * Given a binary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).
 *
 * For example:
 * Given binary tree [3,9,20,null,null,15,7],
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * return its level order traversal as:
 * [
 *   [3],
 *   [9,20],
 *   [15,7]
 * ]
 *
 */

use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root.unwrap());
        while !queue.is_empty() {
            let len = queue.len();
            let mut tv: Vec<i32> = Vec::new();
            for _ in 0..len {
                let node = queue.pop_front();
                tv.push(node.as_ref().unwrap().borrow().val);
                if let Some(l) = node.as_ref().unwrap().borrow().left.clone() {
                    queue.push_back(l.clone());
                };
                if let Some(r) = node.as_ref().unwrap().borrow().right.clone() {
                    queue.push_back(r.clone());
                };
            }
            res.push(tv);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn level_order() {
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }
}