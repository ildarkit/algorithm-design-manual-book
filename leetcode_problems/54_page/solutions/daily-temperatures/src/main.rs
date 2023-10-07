use rand::distributions::{Distribution, Uniform};

struct Solution;

impl Solution {
    pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
        match temp.len() {
            0 => vec![],
            1 => vec![0],
            _ => {
                let mut answer: Vec<i32> = vec![0; temp.len()]; 
                let mut i = 0;
                let mut c;
                while i < temp.len() - 1 {
                    c = Self::sub_interval(
                        &temp[i..],
                        &mut answer[i..]
                    );
                    if c == 0 {
                        break;
                    }
                    i += c;
                    while i < temp.len() - 1 && answer[i] > 0 {
                        i += 1;
                    }
                }
                answer
            }
        }
    }

    fn sub_interval(temp: &[i32], answer: &mut[i32]) -> usize {
        let mut i = 1;
        let mut counter = 1;
        let mut c;
        let mut skip_step = 0;
        let mut j;
        let mut skip;
        let mut last_skip_pos= 0;
        let mut after_skip_pos = 0;

        while i < temp.len() {
            skip = false;
            j = i;
            if j < temp.len() - 2 {
                if last_skip_pos > 0 && temp[last_skip_pos] != temp[j] && temp[j - 1] == temp[j] {
                    c = Self::sub_interval(&temp[(j - 1)..], &mut answer[(j - 1)..]);
                    i += c - 1;
                    if counter == 1 {
                        counter += c;
                    }
                }
                else if temp[last_skip_pos] == temp[j] || temp[j - 1] == temp[j] && answer[j] == 0 {
                    let mut pos = match temp[last_skip_pos] == temp[j] {
                        true => last_skip_pos,
                        false => j - 1,
                    };
                    while j < temp.len() && temp[pos] == temp[j] {
                        skip_step += j - pos;
                        pos = j;
                        j += 1;
                        skip = true;
                    }
                    last_skip_pos = pos;
                }
            }
            if skip {
                i = j;
                after_skip_pos = i;
                // counter += skip_step - c;
            }

            c = 0;
            if i < temp.len() - 1 && temp[0] > temp[i] {
                c = Self::sub_interval(&temp[i..], &mut answer[i..]);
                i += c;
                if counter == 1 {
                    counter += c;
                }
            }

            if i < temp.len() && skip_step > 0 {
                if temp[last_skip_pos] < temp[i] {
                    let mut n = temp[after_skip_pos..=i]
                        .iter()
                        .position(|&x| x > temp[last_skip_pos])
                        .unwrap() + 1;
                    let larger_pos = last_skip_pos + n;

                    if n + skip_step < larger_pos {
                        skip_step += 1;
                    }
                    let before_skip_step = larger_pos - skip_step - 1;
                    if last_skip_pos - skip_step >= before_skip_step {
                        skip_step += before_skip_step;
                    }

                    let mut k = last_skip_pos;
                    while skip_step > 0 {
                        if answer[k] == 0 && temp[k] == temp[last_skip_pos] {
                            answer[k] = n as i32;
                        }
                        skip_step -= 1;
                        k -= 1;
                        n += 1;
                    }
                }
                let mut incremented_step = false;
                while i < temp.len() - 1 && temp[last_skip_pos] == temp[i] {
                    skip_step += i - last_skip_pos;
                    last_skip_pos = i;
                    after_skip_pos = i + 1;
                    i += 1;
                    incremented_step = true;
                }
                if incremented_step {
                    i -= 1;
                }
                c = 0;
            }

            if i < temp.len() && temp[0] < temp[i] {
                if answer[0] == 0 {
                    answer[0] = i as i32;
                }
                while i < temp.len() - 1 && temp[i] < temp[i + 1] {
                    if answer[i] == 0 {
                        answer[i] = 1;
                    }
                    i += 1;
                }
                break;
            }
            if c == 0 {
                i += 1;
            }
        }
        counter
    }
}

pub fn naive_daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![0; temperatures.len()];
    for i in 0..temperatures.len() - 1 {
        for j in (i + 1)..temperatures.len() {
            if temperatures[i] < temperatures[j] {
                answer[i] = (j - i) as i32;
                break;
            }
        }
    }
    answer
}

fn main() {
    // // let mut temperatures = vec![99; 100000];
    // // temperatures[99999] = 100;
    // let temperatures = vec![
    //     100, 41, 83, 57, 57, 83, 84, 57, 63, 52, 57, 78, 91, 41, 34, 75
    // ];
    // println!("temperatures = {:?}", temperatures);
    // let solution = Solution::daily_temperatures(temperatures.clone());
    // println!("solution: {:?}", solution);
    // let naive_solution = naive_daily_temperatures(temperatures);
    // println!("naive solution: {:?}", naive_solution);
    // assert_eq!(solution, vec![0, 1, 4, 2, 1, 1, 6, 1, 3, 1, 1, 1, 0, 2, 1, 0]);

    // let temperatures = vec![
    //     vec![77,77,77,77,77,41,77,41,41,77],
    //     vec![73,74,75,71,69,72,76,73],
    //     vec![50, 72, 41, 90, 31, 81, 40, 32, 33, 81, 47],
    //     vec![64, 35, 74, 46, 74, 52, 52, 81, 98, 71, 69],
    //     vec![33, 74, 94, 56, 85, 97, 72, 81, 50, 85, 42],
    //     vec![86, 65, 85, 90, 65, 32, 83, 83, 81, 91, 100],
    //     vec![31, 100, 91, 50, 83, 87, 76, 43, 31, 31, 30],
    //     vec![95, 86, 74, 74, 30, 71, 76, 84, 67, 38, 42],
    //     vec![78, 93, 42, 62, 62, 83, 83, 92, 99, 58],
    //     vec![88, 33, 37, 74, 37, 49, 39, 53, 43, 43],
    //     vec![91, 62, 94, 84, 36, 43, 65, 85, 85, 61, 99],
    //     vec![34, 85, 91, 46, 40, 79, 53, 62, 62, 84, 72],
    //     vec![50, 94, 48, 98, 98, 64, 98, 33, 47, 99, 95],
    //     vec![87, 47, 77, 70, 70, 69, 49, 35, 70, 99, 59],
    //     vec![87, 98, 98, 67, 94, 41, 41, 65, 85, 53, 58],
    //     vec![78, 78, 78, 59, 55, 59, 59, 55, 37, 45, 93],
    //     vec![100, 73, 63, 95, 42, 42, 40, 42, 42, 31, 58],
    //     vec![95, 95, 37, 79, 79, 41, 70, 47, 62, 98, 63],
    //     vec![94, 94, 48, 39, 66, 71, 82, 39, 94, 44, 100],
    //     vec![68, 44, 69, 88, 54, 38, 54, 48, 54, 54, 92],
    //     vec![62, 74, 54, 88, 88, 41, 75, 99, 74, 84, 86],
    //     vec![42, 40, 67, 76, 76, 45, 61, 30, 44, 76, 76],
    //     vec![85, 68, 85, 60, 81, 85, 85, 41, 74, 89, 91],
    //     vec![86, 74, 77, 54, 35, 32, 34, 77, 58, 87, 75],
    //     vec![100, 41, 83, 57, 57, 83, 84, 57, 63, 52, 57, 78, 91, 41, 34, 75],
    // ];
    // for t in temperatures {
    //     println!("temperatures = {:?}", t);
    //     let solution = Solution::daily_temperatures(t.clone());
    //     println!("solution: {:?}", solution);
    //     let naive_solution = naive_daily_temperatures(t);
    //     println!("naive solution: {:?}", naive_solution);
    //     assert_eq!(solution, naive_solution);
    // }

    let dist = Uniform::new_inclusive(30, 100);
    let mut rng = rand::thread_rng();
    for i in 0..100 {
        if i % 10 == 0 {
            println!("step = {}", i);
        }
        let temperatures: Vec<_> = (1..=10000).map(|_| dist.sample(&mut rng)).collect();
        // println!("temperatures = {:?}", temperatures);
        let solution = Solution::daily_temperatures(temperatures.clone());
        // println!("solution: {:?}", solution);
        let naive_solution = naive_daily_temperatures(temperatures);
        // println!("naive solution: {:?}", naive_solution);
        assert_eq!(solution, naive_solution);
    }
}
