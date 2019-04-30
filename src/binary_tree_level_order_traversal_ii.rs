use super::leetcode::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root == None {
            return vec![];
        }
        let mut vec: Vec<Vec<i32>> = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(root.unwrap());
        while queue.len() > 0 {
            let len = queue.len();
            let mut temp: Vec<i32> = Vec::new();
            for _ in 0..len {
                let node = queue.pop_front();
                temp.push(node.as_ref().unwrap().borrow().val);
                if let Some(l) = node.as_ref().unwrap().borrow().left.clone() {
                    queue.push_back(l.clone());
                };

                if let Some(r) = node.unwrap().borrow().right.clone() {
                    queue.push_back(r.clone());
                }
            }
            vec.insert(0, temp);
        }
        vec
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    use crate::binarytree;
    #[test]
    fn level_order_bottom() {
        assert_eq!(
            vec![vec![15, 7], vec![9, 20], vec![3]],
            Solution::level_order_bottom(binarytree![3, 9, 20, null, null, 15, 7])
        );
        assert_eq!(
            vec![] as Vec<Vec<i32>>,
            Solution::level_order_bottom(binarytree![])
        );
        assert_eq!(vec![vec![1]], Solution::level_order_bottom(binarytree![1]));
        assert_eq!(
            vec![vec![4, 8], vec![6, 7], vec![2, 3], vec![1]],
            Solution::level_order_bottom(binarytree![1, 2, 3, null, 6, null, 7, 4, null, null, 8])
        );
    }
}
