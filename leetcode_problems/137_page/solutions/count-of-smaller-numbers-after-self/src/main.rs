struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let min_num = nums.iter().min().unwrap();
        let max_num = nums.iter().max().unwrap();
        let mut answer = vec![0; nums.len()];
        let mut fenwick_tree = vec![0; (max_num + 2 - min_num) as usize];
        for (j, n) in nums.iter().rev().enumerate() {
            let n = (n - min_num + 1) as usize;
            let mut sum = 0;
            let mut i = n;
            while i > 0 {
                sum += fenwick_tree[i];
                i -= i & i.wrapping_neg();
            }
            answer[nums.len() - 1 - j] = sum;
            let mut i = n + 1;
            while i < fenwick_tree.len() {
                fenwick_tree[i] += 1;
                i += i & i.wrapping_neg();
            }
        }
        answer
    }
}

fn main() {
    let mut nums;

    nums = vec![-1];
    assert_eq!(Solution::count_smaller(nums), vec![0]);

    nums = vec![-1, -1];
    assert_eq!(Solution::count_smaller(nums), vec![0, 0]);

    nums = vec![5, 2, 6, 1, 0, 2];
    assert_eq!(Solution::count_smaller(nums), vec![4, 2, 3, 1, 0, 0]);

    nums = vec![5, 4, 3, 2, 1, 3];
    assert_eq!(Solution::count_smaller(nums), vec![5, 4, 2, 1, 0, 0]);
}
