struct Solution;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        unsafe {
            let result = num.as_mut_vec();
            let mut i = 0;
            let mut count = 0;
            while count < k {
                if result[i] < result[i + 1] {
                    let tmp = result[i];
                    result[i] = result[i + 1];
                    result[i + 1] = tmp;
                }
                if i < result.len() - 1 {
                    i += 1;
                }
                count += 1;
            }
            std::str::from_utf8(&result[i..])
                .unwrap()
                .to_string()
        } 
    }
}

fn main() {
    let k = 3;
    let num = String::from("1432219");
    assert_eq!(String::from("1219"), Solution::remove_kdigits(num, k));
}
