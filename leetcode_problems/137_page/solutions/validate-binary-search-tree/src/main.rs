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
                let root_val = root.borrow().val;
                Solution::check_bst(root.borrow().left.clone(), |val| val >= root_val)
                    && Solution::check_bst(root.borrow().right.clone(), |val| val <= root_val)
            }
            None => true,
        }
    }

    fn check_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        is_invalid_val: impl Fn(i32) -> bool,
    ) -> bool {
        match root {
            Some(ref root) => {
                let mut tree = VecDeque::new();
                tree.push_back(root.clone());
                while let Some(node) = tree.pop_front() {
                    if is_invalid_val(node.borrow().val) {
                        return false;
                    }
                    if let Some(ref left) = node.borrow().left {
                        if left.borrow().val > node.borrow().val {
                            return false;
                        }
                        tree.push_back(left.clone());
                    }
                    if let Some(ref right) = node.borrow().right {
                        if right.borrow().val <= node.borrow().val {
                            return false;
                        }
                        tree.push_back(right.clone());
                    }
                }
                true
            }
            None => true,
        }
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

    // root = TreeNode::new(1);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));

    // [32,26,47,19,null,null,56,null,27]
    root = TreeNode::new(32);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(26))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(47))));
    if let Some(ref left) = root.left {
        let mut node = TreeNode::new(19);
        node.right = Some(Rc::new(RefCell::new(TreeNode::new(27))));
        left.borrow_mut().left = Some(Rc::new(RefCell::new(node)));
    }
    if let Some(ref right) = root.right {
        right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(56))));
    }
    assert!(
        !(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))))
    );
}
