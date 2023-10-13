struct Solution;

impl Solution {
    pub fn remove_kdigits(mut num: String, k: i32) -> String {
        if k as usize == num.len() || num.len() == 1 {
            return "0".to_string()
        }
        let mut result = unsafe {
            let num_vec = num.as_mut_vec();
            let mut i = 0;
            let mut j = 1;
            let mut count = 0;
            while count < k {
                if num_vec[i] == num_vec[j] {
                    if j < num_vec.len() - 1 {
                        j += 1;
                    }
                } else {
                    if num_vec[i] < num_vec[j] {
                        let tmp = num_vec[i];
                        num_vec[i] = num_vec[j];
                        num_vec[j] = tmp;
                    }
                    if i < num_vec.len() - 2 {
                        i += 1;
                        if j < num_vec.len() - 1 {
                            j += 1;
                        }
                    }
                    count += 1;
                }
                if j == num_vec.len() - 1 && num_vec[i] == num_vec[j] && count < k {
                    i += k as usize - count as usize;
                    count = k;
                }
            }
            std::str::from_utf8(&num_vec[i..])
                .unwrap()
                .chars()
                .skip_while(|c| *c == '0')
                .collect::<String>()
        };
        if result.len() == 0 {
            result.push('0')
        }
        result
    }
}

fn main() {
    let k = 1;
    let num = String::from("11");
    assert_eq!(String::from("11"), Solution::remove_kdigits(num, k));
}
