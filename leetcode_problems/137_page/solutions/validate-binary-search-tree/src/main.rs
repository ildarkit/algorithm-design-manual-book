use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref node) = root {
            if let Some(ref left) = node.borrow().left {
                if left.borrow().val > node.borrow().val {
                    return false;
                }
            }
            if let Some(ref right) = node.borrow().right {
                if right.borrow().val < node.borrow().val {
                    return false;
                }
            }
            Solution::is_valid_bst(node.borrow().left.clone()) &&
            Solution::is_valid_bst(node.borrow().right.clone())
        } else {
            true
        }
    }
}

fn main() {
    let mut root;

    root = TreeNode::new(5);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    if let Some(ref right) = root.right {
        right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    }
    assert!(
        !(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))))
    );
}
