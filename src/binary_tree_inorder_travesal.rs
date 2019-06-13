/*
 * 94. Binary Tree Inorder Traversal
 *
 * Given a binary tree, return the inorder traversal of its nodes' values.
 *
 * Example:
 * Input: [1,null,2,3]
 *    1
 *     \
 *      2
 *     /
 *    3
 *
 * Output: [1,3,2]
 *
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */

use crate::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        Self::recursive_helper(&root, &mut v);
        v
    }

    fn recursive_helper(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::recursive_helper(&node.borrow().left, v);
            v.push(node.borrow().val);
            Self::recursive_helper(&node.borrow().right, v);
        }
    }

    pub fn inorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = Vec::new();
        let mut stack = Vec::new();
        let mut cur = root.clone();

        while cur.is_some() || !stack.is_empty() {
            while cur.is_some() {
                let temp = cur.as_ref().unwrap().borrow().left.clone();
                stack.push(cur);
                cur = temp;
            }

            let temp = stack.pop().unwrap();
            v.push(temp.as_ref().unwrap().borrow().val);
            cur = temp.as_ref().unwrap().borrow().right.clone();
        }
        v
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn inorder_traversal() {
        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traversal(binarytree![1, null, 2, 3])
        );

        assert_eq!(
            vec![6, 2, 8, 8, 3, 9, 5, 1],
            Solution::inorder_traversal(binarytree![1, 8, null, 8, 9, 2, null, 3, 5, 6])
        );

        assert_eq!(
            vec![] as Vec<i32>,
            Solution::inorder_traversal(binarytree![])
        );
    }
}