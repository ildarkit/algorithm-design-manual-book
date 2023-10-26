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
                            && Solution::check_bst(left.clone(), root.borrow().val, true)
                    }
                    None => true,
                };
                left_is_valid
                    && match root.borrow().right {
                        Some(ref right) => {
                            right.borrow().val > root.borrow().val
                                && Solution::check_bst(right.clone(), root.borrow().val, false)
                        }
                        None => true,
                    }
            }
            None => true,
        }
    }
    fn check_bst(tree: Rc<RefCell<TreeNode>>, root_val: i32, is_left_tree: bool) -> bool {
        let mut queue = VecDeque::new();
        let mut max_val = root_val;
        let mut min_val = root_val;
        queue.push_back(tree.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(ref left) = node.borrow().left {
                if left.borrow().val >= node.borrow().val {
                    return false;
                }
                if ((is_left_tree && min_val != root_val) || !is_left_tree)
                    && left.borrow().val <= min_val
                {
                    return false;
                }
                if is_left_tree {
                    queue.push_front(left.clone());
                } else {
                    queue.push_back(left.clone());
                }
            } else if node.borrow().right.is_some() {
                min_val = node.borrow().val;
            }
            if let Some(ref right) = node.borrow().right {
                if right.borrow().val <= node.borrow().val {
                    return false;
                }
                if max_val != root_val && right.borrow().val >= max_val {
                    return false;
                }
                if is_left_tree {
                    queue.push_front(right.clone());
                } else {
                    queue.push_back(right.clone());
                }
            } else {
                max_val = node.borrow().val;
            }
        }
        true
    }
}

fn main() {
    let mut root;

    // root = TreeNode::new(5);
    // root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // if let Some(ref right) = root.right {
    //     right.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    //     right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    // }
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));
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
    //
    // // [3,null,30,10,null,null,15,null,45]
    // //      3
    // //       \
    // //       30
    // //      /
    // //     10
    // //      \
    // //      15
    // //       \
    // //       45
    // let mut right_child_child_child = TreeNode::new(15);
    // right_child_child_child.right = Some(Rc::new(RefCell::new(TreeNode::new(45))));
    // let mut left_child_child = TreeNode::new(10);
    // left_child_child.right = Some(Rc::new(RefCell::new(right_child_child_child)));
    // let mut right_child = TreeNode::new(30);
    // right_child.left = Some(Rc::new(RefCell::new(left_child_child)));
    // root = TreeNode::new(3);
    // root.right = Some(Rc::new(RefCell::new(right_child)));
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));
    //
    // // [24,-60,null,-60,-6]
    // //      24
    // //     /
    // //   -60
    // //  /   \
    // //-60   -6
    // let mut left_child = TreeNode::new(-60);
    // left_child.left = Some(Rc::new(RefCell::new(TreeNode::new(-60))));
    // left_child.right = Some(Rc::new(RefCell::new(TreeNode::new(-6))));
    // root = TreeNode::new(24);
    // root.left = Some(Rc::new(RefCell::new(left_child)));
    // assert!(!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root))))));

    // [98,97,null,88,null,84,null,79,87,64,null,null,null,63,69,62,null,null,null,30,
    // null,27,59,9,null,null,null,3,null,0,null,-4,null,-16,null,-18,-7,-19,null,null,
    // null,-23,null,-34,null,-42,null,-59,null,-63,null,-64,null,-69,null,-75,null,-81]
    //
    //                                                              98
    //                                                             /
    //                                                            97
    //                                                           /
    //                                                          88
    //                                                         /
    //                                                        84
    //                                                       /  \
    //                                                      79  87
    //                                                     /
    //                                                    64
    //                                                   /  \
    //                                                  63  69
    //                                                 /
    //                                                62
    //                                               /
    //                                              30
    //                                             /  \
    //                                            27  59
    //                                           /
    //                                          9
    //                                         /
    //                                        3
    //                                       /
    //                                      0
    //                                     /
    //                                    -4
    //                                   /
    //                                 -16
    //                                 /  \
    //                               -18  -7
    //                               /
    //                             -19
    //                             /
    //                           -23
    //                           /
    //                         -34
    //                         /
    //                       -42
    //                       /
    //                     -59
    //                     /
    //                   -63
    //                   /
    //                 -64
    //                 /
    //               -69
    //               /
    //             -75
    //             /
    //           -81
    let node25 = TreeNode::new(-81);
    let mut node24 = TreeNode::new(-75);
    node24.left = Some(Rc::new(RefCell::new(node25)));
    let mut node23 = TreeNode::new(-69);
    node23.left = Some(Rc::new(RefCell::new(node24)));
    let mut node22 = TreeNode::new(-64);
    node22.left = Some(Rc::new(RefCell::new(node23)));
    let mut node21 = TreeNode::new(-63);
    node21.left = Some(Rc::new(RefCell::new(node22)));
    let mut node20 = TreeNode::new(-59);
    node20.left = Some(Rc::new(RefCell::new(node21)));
    let mut node19 = TreeNode::new(-42);
    node19.left = Some(Rc::new(RefCell::new(node20)));
    let mut node18 = TreeNode::new(-34);
    node18.left = Some(Rc::new(RefCell::new(node19)));
    let mut node17 = TreeNode::new(-23);
    node17.left = Some(Rc::new(RefCell::new(node18)));
    let mut node16 = TreeNode::new(-19);
    node16.left = Some(Rc::new(RefCell::new(node17)));
    let mut node15 = TreeNode::new(-18);
    node15.left = Some(Rc::new(RefCell::new(node16)));
    let mut node14 = TreeNode::new(-16);
    node14.left = Some(Rc::new(RefCell::new(node15)));
    node14.right = Some(Rc::new(RefCell::new(TreeNode::new(-7))));
    let mut node13 = TreeNode::new(-4);
    node13.left = Some(Rc::new(RefCell::new(node14)));
    let mut node12 = TreeNode::new(0);
    node12.left = Some(Rc::new(RefCell::new(node13)));
    let mut node11 = TreeNode::new(3);
    node11.left = Some(Rc::new(RefCell::new(node12)));
    let mut node10 = TreeNode::new(9);
    node10.left = Some(Rc::new(RefCell::new(node11)));
    let mut node9 = TreeNode::new(27);
    node9.left = Some(Rc::new(RefCell::new(node10)));
    let mut node8 = TreeNode::new(30);
    node8.left = Some(Rc::new(RefCell::new(node9)));
    node8.right = Some(Rc::new(RefCell::new(TreeNode::new(59))));
    let mut node7 = TreeNode::new(62);
    node7.left = Some(Rc::new(RefCell::new(node8)));
    let mut node6 = TreeNode::new(63);
    node6.left = Some(Rc::new(RefCell::new(node7)));
    let mut node5 = TreeNode::new(64);
    node5.left = Some(Rc::new(RefCell::new(node6)));
    node5.right = Some(Rc::new(RefCell::new(TreeNode::new(69))));
    let mut node4 = TreeNode::new(79);
    node4.left = Some(Rc::new(RefCell::new(node5)));
    let mut node3 = TreeNode::new(84);
    node3.left = Some(Rc::new(RefCell::new(node4)));
    node3.right = Some(Rc::new(RefCell::new(TreeNode::new(87))));
    let mut node2 = TreeNode::new(88);
    node2.left = Some(Rc::new(RefCell::new(node3)));
    let mut node1 = TreeNode::new(97);
    node1.left = Some(Rc::new(RefCell::new(node2)));
    root = TreeNode::new(98);
    root.left = Some(Rc::new(RefCell::new(node1)));
    assert!(Solution::is_valid_bst(Some(Rc::new(RefCell::new(root)))));
}
