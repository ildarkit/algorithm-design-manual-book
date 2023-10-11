#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32)
        -> Option<Box<ListNode>>
    {
        let len = Solution::length(head.as_deref());
        println!("head = {:?}", to_vector(head.as_deref()));
        let k = k as usize % len;

        match k == 0 {
            true => head,
            false => {
                let mut i = 1;
                let mut node = head.as_deref_mut().unwrap();
                while i < len - k {
                    node = node.next.as_deref_mut().unwrap();
                    i += 1;
                }
                let mut new_head = node.next.take();
                println!("pred node = {:?}", to_vector(Some(&node.clone())));
                println!("old head = {:?}", to_vector(head.as_deref()));
                println!("new head = {:?}", to_vector(new_head.as_deref()));
                let mut j = 1;
                node = new_head.as_deref_mut().unwrap();
                while j < len - i {
                    node = node.next.as_deref_mut().unwrap();
                    j += 1;
                }
                println!("last node = {:?}", to_vector(Some(&node.clone())));
                node.next = head;
                println!("new head = {:?}", to_vector(new_head.as_deref()));
                new_head
            },
        }
    }

    fn length(mut head: Option<&ListNode>) -> usize {
        let mut i = 0;
        while let Some(node) = head {
            head = node.next.as_deref();
            i += 1;
        }
        i
    }
}

fn to_vector(mut head: Option<&ListNode>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = head {
        v.push(node.val);
        head = node.next.as_deref();
    }
    v
}

fn main() {
    let k = 7;
    let mut input_value = vec![0, 1, 2, 3, 4, 5];
    let len = input_value.len();
    let mut head = Box::new(ListNode::new(input_value[len - 1]));
    for i in input_value[..len - 1].iter().rev() {
        let mut node = Box::new(ListNode::new(*i));
        node.next = Some(head);
        head = node;
    }
    let node = Solution::rotate_right(Some(head), k);
    let mut node = node.as_ref();
    let mut answer = vec![];
    while let Some(n) = node {
        answer.push(n.val);
        node = n.next.as_ref();
    }
    input_value.rotate_right(k as usize % len);
    assert_eq!(answer, input_value);
}
