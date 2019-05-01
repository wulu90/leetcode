use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(v) => match (v.borrow().left.clone(), v.borrow().right.clone()) {
                (Some(left), Some(right)) => {
                    1 + std::cmp::min(Self::min_depth(Some(left)), Self::min_depth(Some(right)))
                }
                (Some(left), None) => 1 + Self::min_depth(Some(left)),
                (None, Some(right)) => 1 + Self::min_depth(Some(right)),
                (None, None) => 1,
            },
            None => 0,
        }
    }

    pub fn min_depth_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), 1));
        while !queue.is_empty() {
            if let Some((Some(v), level)) = queue.pop_front() {
                if v.borrow().left.is_none() && v.borrow().right.is_none() {
                    return level;
                }
                queue.push_back((v.borrow().left.clone(), level + 1));
                queue.push_back((v.borrow().right.clone(), level + 1));
            }
        }
        0
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn min_depth() {
        assert_eq!(0, Solution::min_depth(binarytree![]));
        assert_eq!(1, Solution::min_depth(binarytree![1]));
        assert_eq!(2, Solution::min_depth(binarytree![1, 2]));
        assert_eq!(2, Solution::min_depth(binarytree![1, 2, 3]));
        assert_eq!(2, Solution::min_depth(binarytree![1, 2, 3, 4]));
        assert_eq!(
            2,
            Solution::min_depth(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }
    #[test]
    fn min_depth_bfs() {
        assert_eq!(0, Solution::min_depth_bfs(binarytree![]));
        assert_eq!(1, Solution::min_depth_bfs(binarytree![1]));
        assert_eq!(2, Solution::min_depth_bfs(binarytree![1, 2]));
        assert_eq!(2, Solution::min_depth_bfs(binarytree![1, 2, 3]));
        assert_eq!(2, Solution::min_depth_bfs(binarytree![1, 2, 3, 4]));
        assert_eq!(
            2,
            Solution::min_depth_bfs(binarytree![3, 9, 20, null, null, 15, 7])
        );
    }
}
