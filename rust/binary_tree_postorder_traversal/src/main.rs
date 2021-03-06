// Accepted
// @lc id=145 lang=rust
// problem: binary_tree_postorder_traversal
extern crate util;
use util::binary_tree::*;

struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ret = vec![];
        let mut stack = vec![];
        stack.push(root.unwrap());
        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let left = top.borrow().left.clone();
            let right = top.borrow().right.clone();
            if left.is_none() && right.is_none() {
                ret.push(top.borrow().val);
                continue;
            }
            stack.push(Rc::new(RefCell::new(TreeNode::new(top.borrow().val))));
            if right.is_some() {
                stack.push(right.unwrap());
            }
            if left.is_some() {
                stack.push(left.unwrap());
            }
        }
        return ret;
    }
}

fn main() {
    let root = BinaryTree::new(vec![Some(1), None, Some(2), Some(3)]);
    println!("{:?}", Solution::postorder_traversal(root));
}
