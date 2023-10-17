struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n= n as usize;
        let mut ans = vec![0; n + 1];
        for i in 1..=n {
            // 2 ^ n
            if i > 1 && i & (i - 1) == 0 {
                ans[i] = 1;
            // even
            } else if i & 1 == 0 {
                ans[i] = ans[i >> 1];
            // odd
            } else {
                ans[i] = ans[i - 1] + 1;
            }
        }
        ans
    }
}

fn main() {
    let n = 259;
    println!("{:?}", Solution::count_bits(n));
}
