struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let zero_num = "0".to_string();
        if k as usize == num.len() || num.len() == 1 {
            return zero_num;
        }
        let mut result_num = Vec::with_capacity(num.len() - k as usize);
        let mut count = 0;

        for current in num.chars() {
            while result_num
                .last()
                .map_or(false, |num_char| *num_char > current && count < k)
            {
                count += 1;
                result_num.pop();
            }
            result_num.push(current);
        }
        while count < k {
            count += 1;
            result_num.pop();
        }
        let mut num = result_num
            .into_iter()
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

    k = 8;
    num = String::from("9999999999991");
    assert_eq!(String::from("99991"), Solution::remove_kdigits(num, k));

    k = 9;
    num = String::from("1234567890");
    assert_eq!(String::from("0"), Solution::remove_kdigits(num, k));

    k = 3;
    num = String::from("1432219");
    assert_eq!(String::from("1219"), Solution::remove_kdigits(num, k));

    k = 2;
    num = String::from("12345");
    assert_eq!(String::from("123"), Solution::remove_kdigits(num, k));

    k = 1;
    num = String::from("112");
    assert_eq!(String::from("11"), Solution::remove_kdigits(num, k));

    k = 1;
    num = String::from("102");
    assert_eq!(String::from("2"), Solution::remove_kdigits(num, k));

    k = 1;
    num = String::from("100");
    assert_eq!(String::from("0"), Solution::remove_kdigits(num, k));

    k = 2;
    num = String::from("54321");
    assert_eq!(String::from("321"), Solution::remove_kdigits(num, k));
}
