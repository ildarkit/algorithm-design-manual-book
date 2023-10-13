struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let zero_num = "0".to_string();
        if k as usize == num.len() || num.len() == 1 {
            return zero_num;
        }
        let mut result_num = Vec::with_capacity(num.len() - k as usize);
        let mut i = 0;
        let mut count = 0;
        for (j, num_char) in num.as_bytes().windows(2).enumerate() {
            if num_char[0] > num_char[1] {
                loop {
                    count += 1;
                    match result_num.last() {
                        Some(c) if *c > num_char[1] && count < k => {
                            result_num.pop();
                        },
                        Some(_) | None => break,
                    }
                }
            } else {
                result_num.push(num_char[0]);
            }
            if count == k {
                if result_num.len() > 0 {
                    i = j + 2;
                } else {
                    i = j + 1;
                }
                break;
            } else if result_num.len() > 0 && j == num.len() - 1 - k as usize {
                break;
            }
        }

        if result_num.len() == 0 && i == 0 {
            return zero_num;
        }

        let num_str = if result_num.len() == 0 {
            &num.as_str()[i..]
        } else {
            if i > 0 {
                result_num.extend_from_slice(&num.as_bytes()[i..]);
            }
            std::str::from_utf8(&result_num[..]).unwrap()
        };

        let mut num = num_str
            .chars()
            .skip_while(|c| *c == '0')
            .collect::<String>();
        if num.len() == 0 {
            num = zero_num;
        }
        num

    }
}

fn main() {
    let mut k;
    let mut num;

    k = 3;
    num = String::from("1432219");
    assert_eq!(String::from("1219"), Solution::remove_kdigits(num, k));

    // k = 2;
    // num = String::from("12345");
    // assert_eq!(String::from("123"), Solution::remove_kdigits(num, k));
    //
    // k = 1;
    // num = String::from("112");
    // assert_eq!(String::from("11"), Solution::remove_kdigits(num, k));
    //
    // k = 1;
    // num = String::from("102");
    // assert_eq!(String::from("2"), Solution::remove_kdigits(num, k));
    //
    // k = 1;
    // num = String::from("100");
    // assert_eq!(String::from("0"), Solution::remove_kdigits(num, k));
    //
    // k = 2;
    // num = String::from("54321");
    // assert_eq!(String::from("321"), Solution::remove_kdigits(num, k));
}
