struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        Solution::ksum(&nums[..], target, 4)
    }

    fn ksum(nums: &[i32], target: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if nums.len() == 0 {
            return res;
        }
        let average_value = target / k;
        if average_value < nums[0] || nums.last().unwrap() < &average_value {
            return res;
        }
        if k == 2 {
            return Solution::two_sum(nums, target);
        }
        for i in 0..nums.len() {
            if i == 0 || nums[i - 1] != nums[i] {
                if let Some(new_target) = target.checked_sub(nums[i]) {
                    for subset in Solution::ksum(&nums[(i + 1)..], new_target, k - 1) {
                        res.push([vec![nums[i]], subset].concat());
                    }
                }
            }
        }
        res
    }

    fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let current_sum = nums[lo] + nums[hi];
            if current_sum < target || (lo > 0 && nums[lo] == nums[lo - 1]) {
                lo += 1;
            } else if current_sum > target || (hi < nums.len() - 1 && nums[hi] == nums[hi + 1]) {
                hi -= 1;
            } else {
                res.push(vec![nums[lo], nums[hi]]);
                lo += 1;
                hi -= 1;
            }
        }
        res
    }
}

fn main() {
    let mut target;
    let mut nums;

    target = 0;
    nums = vec![1, 0, -1, 0, -2, 2];
    assert_eq!(Solution::four_sum(nums, target),
               vec![[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
    );

    target = 294967296;
    nums = vec![-1000000000,-1000000000,1000000000,-1000000000,-1000000000];
    assert_eq!(Solution::four_sum(nums, target), Vec::<Vec<i32>>::new());

    target = 0;
    nums = vec![1000000000,1000000000,1000000000,1000000000,
                -1000000000,-1000000000,-1000000000,-1000000000];
    assert_eq!(Solution::four_sum(nums, target),
             vec![[-1000000000, -1000000000, 1000000000, 1000000000]]);
}
