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
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        unimplemented!()
    }
}

fn tree_to_vec(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    unimplemented!()
}

fn main() {
    let mut preorder;
    let mut inorder;
    let mut tree;

    preorder = vec![3, 9, 20, 15, 7];
    inorder = vec![9, 3, 15, 20, 7];
    tree = vec![Some(3), Some(9), Some(20),
                None, None, Some(15), Some(7)];
    assert_eq!(
        tree_to_vec(Solution::build_tree(preorder, inorder)),
        tree
    );
}
