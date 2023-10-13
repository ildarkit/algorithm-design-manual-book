struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        if k as usize == num.len() || num.len() == 1 {
            return "0".to_string()
        }
        let mut result_num = Vec::with_capacity(num.len() - k as usize);
        let mut i = 0;
        let mut count = 0;
        for (j, pair) in num.as_bytes().windows(2).enumerate() {
            if pair[0] > pair[1] {
                while let Some(c) = result_num.last() {
                    if *c > pair[1] && count < k {
                        result_num.pop();
                    } else {
                        break;
                    }
                    count += 1;
                }
            } else {
                result_num.push(pair[0]);
                if j == num.len() - 2 {
                    result_num.push(pair[1]);
                }
            }
            if count == k {
                i = j + 1;
                break;
            }
        }
        while count < k {
            result_num.pop();
            count += 1;
        }
        if i > 0 {
            result_num.extend_from_slice(&num.as_bytes()[i..]);
        }
        let mut result = std::str::from_utf8(&result_num[..])
            .unwrap()
            .chars()
            .skip_while(|c| *c == '0')
            .collect::<String>();
        if result.len() == 0 {
            result.push('0')
        }
        result
    }
}

fn main() {
    let k = 2;
    let num = String::from("12345");
    assert_eq!(String::from("123"), Solution::remove_kdigits(num, k));
}
