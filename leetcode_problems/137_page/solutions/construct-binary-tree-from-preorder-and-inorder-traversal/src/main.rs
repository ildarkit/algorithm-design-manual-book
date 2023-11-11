use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_subtree(&preorder[..], &inorder[..])
    }

    fn build_subtree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match inorder.len() > 0 {
            true => {
                let mut i = 0;
                let node_pos;
                loop {
                    if let Some(pos) = inorder.iter().position(|&n| n == preorder[i]) {
                        node_pos = pos;
                        break;
                    }
                    i += 1;
                }
                let mut node = TreeNode::new(preorder[i]);
                node.left =
                    Solution::build_subtree(
                        &preorder[(i + 1)..],
                        &inorder[..node_pos]
                    );
                node.right = Solution::build_subtree(
                    &preorder[(i + 1)..],
                    &inorder[(node_pos + 1)..],
                );
                Some(Rc::new(RefCell::new(node)))
            }
            false => None,
        }
    }
}

fn tree_to_vec(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut vec_tree = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(tree.clone());
    while queue.len() > 0 {
        let node = queue.pop_front();
        match node {
            Some(Some(node)) => {
                vec_tree.push(Some(node.borrow().val));
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            }
            _ => vec_tree.push(None),
        }
    }
    let right_pos = vec_tree.iter().rev().skip_while(|&n| n.is_none()).count();
    Vec::from(&vec_tree[..right_pos])
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

    // [3,8,12,null,10,9,13,7,20,null,null,15,null,null,11,14,30]
    preorder = vec![3, 8, 10, 7, 11, 20, 14, 30, 12, 9, 13, 15];
    inorder = vec![8, 7, 11, 10, 14, 20, 30, 3, 9, 12, 15, 13];
    tree = vec![Some(3), Some(8), Some(12), None, Some(10), Some(9),
        Some(13), Some(7), Some(20), None, None, Some(15), None,
        None, Some(11), Some(14), Some(30)];
    assert_eq!(tree_to_vec(Solution::build_tree(preorder, inorder)), tree);
}
