use rand::distributions::{Distribution, Uniform};

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
        if len == 0 {
            return head;
        }
        let k = k as usize % len;

        match k == 0 {
            true => head,
            false => {
                let node = head.as_deref_mut().unwrap();
                let (mut node, i) = Solution::node_pos(node, len - k);
                let mut new_head = node.next.take();

                node = new_head.as_deref_mut().unwrap();
                let (node, _) = Solution::node_pos(node, len - i);

                node.next = head;
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

    fn node_pos(mut head: &mut ListNode, len: usize) -> (&mut ListNode, usize) {
        let mut i = 1;
        while i < len {
            head = head.next.as_deref_mut().unwrap();
            i += 1;
        }
        (head, i)
    }
}

// for debugging
fn _to_vector(mut head: Option<&ListNode>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = head {
        v.push(node.val);
        head = node.next.as_deref();
    }
    v
}

fn main() {
    let maxk = 2_000_000_000;
    let values = Uniform::new_inclusive(-100, 100);
    let len_values = Uniform::new_inclusive(0, 500);
    let kdist = Uniform::new_inclusive(0, maxk);
    let mut rng = rand::thread_rng();
    for j in 0..100_000 {
        if j % 10000 == 0 {
            println!("test steps = {j}");
        }
        let k = kdist.sample(&mut rng);
        let len = len_values.sample(&mut rng);
        let mut input_value: Vec<_> = (0..len)
            .map(|_| values.sample(&mut rng))
            .collect();
        let len = input_value.len();
        let head = match len == 0 {
            true => None,
            false => {
                let mut head = Box::new(
                    ListNode::new(input_value[len - 1])
                );
                for i in input_value[..len - 1].iter().rev() {
                    let mut node = Box::new(ListNode::new(*i));
                    node.next = Some(head);
                    head = node;
                }
                Some(head)
            },
        };
        let node = Solution::rotate_right(head, k);
        let mut node = node.as_ref();
        let mut answer = vec![];
        while let Some(n) = node {
            answer.push(n.val);
            node = n.next.as_ref();
        }
        if len > 0 {
            input_value.rotate_right(k as usize % len);
        }
        assert_eq!(answer, input_value);
    }
}
