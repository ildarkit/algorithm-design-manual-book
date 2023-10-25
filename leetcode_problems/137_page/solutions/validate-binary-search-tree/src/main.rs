use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(ref root) => {
                let left_is_valid = match root.borrow().left {
                    Some(ref left) => {
                        left.borrow().val < root.borrow().val
                            && Solution::check_bst(
                                left.clone(),
                                root.borrow().val,
                                true,
                                |val, root_val| val >= root_val,
                            )
                    }
                    None => true,
                };
                left_is_valid
                    && match root.borrow().right {
                        Some(ref right) => {
                            right.borrow().val > root.borrow().val
                                && Solution::check_bst(
                                    right.clone(),
                                    root.borrow().val,
                                    false,
                                    |val, root_val| val <= root_val,
                                )
                        }
                        None => true,
                    }
            }
            None => true,
        }
    }

    fn check_bst(
        tree: Rc<RefCell<TreeNode>>,
        root_val: i32,
        is_left_tree: bool,
        is_invalid_val: impl Fn(i32, i32) -> bool,
    ) -> bool {
        let mut queue = VecDeque::new();
        let mut new_root_val = root_val;
        queue.push_back(tree.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(ref left) = node.borrow().left {
                if left.borrow().val > node.borrow().val {
                    return false;
                }
                if !is_left_tree && is_invalid_val(left.borrow().val, new_root_val) {
                    return false;
                }
                queue.push_back(left.clone());
            } else if !is_left_tree {
                new_root_val = node.borrow().val;
            }
            if let Some(ref right) = node.borrow().right {
                if right.borrow().val <= node.borrow().val {
                    return false;
                }
                if is_left_tree && is_invalid_val(right.borrow().val, new_root_val) {
                    return false;
                }
                queue.push_back(right.clone());
            } else if is_left_tree {
                new_root_val = node.borrow().val;
            }
        }
        true
    }
}

fn main() {
    let mut root;

    // root = TreeNode::new(5);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // if let Some(ref right) = root.right {
    //     right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    //     right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // }
    // assert!(
    //     !(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))))
    // );
    //
    // root = TreeNode::new(5);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // if let Some(ref right) = root.right {
    //     right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    //     right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    // }
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));
    //
    // root = TreeNode::new(1);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));
    //
    // // [32,26,47,19,null,null,56,null,27]
    // root = TreeNode::new(32);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(26))));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(47))));
    // if let Some(ref left) = root.left {
    //     let mut node = TreeNode::new(19);
    //     node.right = Some(Rc::new(RefCell::new(TreeNode::new(27))));
    //     left.borrow_mut().left = Some(Rc::new(RefCell::new(node)));
    // }
    // if let Some(ref right) = root.right {
    //     right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(56))));
    // }
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));

    // [3,null,30,10,null,null,15,null,45]
    //      3
    //       \
    //       30
    //      /
    //     10
    //      \
    //      15
    //       \
    //       45
    let mut right_child_child_child = TreeNode::new(15);
    right_child_child_child.right = Some(Rc::new(RefCell::new(TreeNode::new(45))));
    let mut left_child_child = TreeNode::new(10);
    left_child_child.right = Some(Rc::new(RefCell::new(right_child_child_child)));
    let mut right_child = TreeNode::new(30);
    right_child.left = Some(Rc::new(RefCell::new(left_child_child)));
    root = TreeNode::new(3);
    root.right = Some(Rc::new(RefCell::new(right_child)));
    assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));
}
