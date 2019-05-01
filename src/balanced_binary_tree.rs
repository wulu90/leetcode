use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::new();
        if let Some(v) = root {
            queue.push_back(v);
            while !queue.is_empty() {
                let cur = queue.pop_front();
                let ld = Self::depth_of_tree(cur.as_ref().unwrap().borrow().left.clone(), 0);
                let rd = Self::depth_of_tree(cur.as_ref().unwrap().borrow().right.clone(), 0);
                if (ld - rd).abs() >= 2 {
                    return false;
                } else {
                    if let Some(left) = cur.as_ref().unwrap().borrow().left.clone() {
                        queue.push_back(left);
                    };
                    if let Some(right) = cur.as_ref().unwrap().borrow().right.clone() {
                        queue.push_back(right);
                    };
                }
            }
            true
        } else {
            true
        }
    }

    fn depth_of_tree(root: Option<Rc<RefCell<TreeNode>>>, level: i32) -> i32 {
        match root {
            Some(v) => match (v.borrow().left.clone(), v.borrow().right.clone()) {
                (Some(l), Some(r)) => {
                    let ld = Self::depth_of_tree(Some(l), level + 1);
                    let rd = Self::depth_of_tree(Some(r), level + 1);
                    return ld.max(rd);
                }
                (Some(l), None) => {
                    let ld = Self::depth_of_tree(Some(l), level + 1);
                    return ld;
                }
                (None, Some(r)) => {
                    let rd = Self::depth_of_tree(Some(r), level + 1);
                    return rd;
                }
                (_, _) => level + 1,
            },
            None => level,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;

    #[test]
    fn is_balanced() {
        assert_eq!(true, Solution::is_balanced(binarytree![]));
        assert_eq!(true, Solution::is_balanced(binarytree![1]));
        assert_eq!(true, Solution::is_balanced(binarytree![1, 2]));
        assert_eq!(
            true,
            Solution::is_balanced(binarytree![3, 9, 20, null, null, 15, 7])
        );
        assert_eq!(
            false,
            Solution::is_balanced(binarytree![1, 2, 2, 3, 3, null, null, 4, 4])
        );

        assert_eq!(
            false,
            Solution::is_balanced(binarytree![1, null, 2, null, 3])
        );

        assert_eq!(
            false,
            Solution::is_balanced(binarytree![1, 2, 2, 3, null, null, 3, 4, null, null, 4])
        );
    }
}
