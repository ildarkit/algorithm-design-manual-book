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
            while j < temp.len() && temp[j - 1] == temp[j] {
                skip_step += 1;
                last_skip_pos = j;
                j += 1;
                skip = true;
            }
            if skip {
                i += skip_step;
                after_skip_pos = i;
                counter += skip_step;
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

                    while skip_step > 0 {
                        if answer[last_skip_pos] == 0 {
                            answer[last_skip_pos] = n as i32;
                        }
                        skip_step -= 1;
                        last_skip_pos -= 1;
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
    // let temperatures = vec![77,77,77,77,77,41,77,41,41,77];
    // let temperatures = vec![73,74,75,71,69,72,76,73];
    // let temperatures = vec![50, 72, 41, 90, 31, 81, 40, 32, 33, 81, 47];
    // let temperatures = vec![64, 35, 74, 46, 74, 52, 52, 81, 98, 71, 69];
    // let temperatures = vec![33, 74, 94, 56, 85, 97, 72, 81, 50, 85, 42];
    // let temperatures = vec![86, 65, 85, 90, 65, 32, 83, 83, 81, 91, 100];
    // let temperatures = vec![31, 100, 91, 50, 83, 87, 76, 43, 31, 31, 30];
    // let temperatures = vec![95, 86, 74, 74, 30, 71, 76, 84, 67, 38, 42];
    // let temperatures = vec![78, 93, 42, 62, 62, 83, 83, 92, 99, 58];
    // let temperatures = vec![88, 33, 37, 74, 37, 49, 39, 53, 43, 43];
    // let temperatures = vec![91, 62, 94, 84, 36, 43, 65, 85, 85, 61, 99];
    // let temperatures = vec![34, 85, 91, 46, 40, 79, 53, 62, 62, 84, 72];
    // let temperatures = vec![50, 94, 48, 98, 98, 64, 98, 33, 47, 99, 95];
    // let temperatures = vec![87, 47, 77, 70, 70, 69, 49, 35, 70, 99, 59];
    // let temperatures = vec![87, 98, 98, 67, 94, 41, 41, 65, 85, 53, 58];
    // let temperatures = vec![78, 78, 78, 59, 55, 59, 59, 55, 37, 45, 93];
    // let temperatures = vec![100, 73, 63, 95, 42, 42, 40, 42, 42, 31, 58];
    let temperatures = vec![95, 95, 37, 79, 79, 41, 70, 47, 62, 98, 63];
    // let mut temperatures = vec![99; 100000];
    // temperatures[99999] = 100;
    println!("temperatures = {:?}", temperatures);
    let solution = Solution::daily_temperatures(temperatures.clone());
    println!("solution: {:?}", solution);
    let naive_solution = naive_daily_temperatures(temperatures);
    println!("naive solution: {:?}", naive_solution);
    // assert_eq!(solution, vec![0, 0, 0, 0, 0, 1, 0, 2, 1, 0]);
    // assert_eq!(solution, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![1, 2, 1, 0, 1, 0, 3, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![2, 1, 5, 1, 3, 2, 1, 1, 0, 0, 0]);
    // assert_eq!(solution, vec![1, 1, 3, 1, 1, 0, 1, 2, 1, 0, 0]);
    // assert_eq!(solution, vec![3, 1, 1, 6, 2, 1, 3, 2, 1, 1, 0]);
    // assert_eq!(solution, vec![1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0]);
    // assert_eq!(solution, vec![0, 0, 4, 3, 1, 1, 1, 0, 0, 1, 0]);
    // assert_eq!(solution, vec![1, 7, 1, 2, 1, 2, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![0, 1, 1, 0, 1, 2, 1, 0, 0, 0]);
    // assert_eq!(solution, vec![2, 1, 8, 4, 1, 1, 1, 3, 2, 1, 0]);
    // assert_eq!(solution, vec![1, 1, 0, 2, 1, 4, 1, 2, 1, 0, 0]);
    // assert_eq!(solution, vec![1, 2, 1, 6, 5, 1, 3, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![9, 1, 7, 6, 5, 3, 2, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![1, 0, 0, 1, 0, 2, 1, 1, 0, 1, 0]);
    // assert_eq!(solution, vec![10, 9, 8, 7, 1, 5, 4, 3, 1, 1, 0]);
    // assert_eq!(solution, vec![0, 2, 1, 0, 6, 5, 1, 3, 2, 1, 0]);
    assert_eq!(solution, vec![9, 8, 1, 6, 5, 1, 3, 1, 1, 0, 0]);
    // assert_eq!(solution, naive_solution);

    // let dist = Uniform::new_inclusive(30, 100);
    // let mut rng = rand::thread_rng();
    // for i in 0..10000 {
    //     if i % 10 == 0 {
    //         println!("step = {}", i);
    //     }
    //     let temperatures: Vec<_> = (1..=11).map(|_| dist.sample(&mut rng)).collect();
    //     println!("temperatures = {:?}", temperatures);
    //     let solution = Solution::daily_temperatures(temperatures.clone());
    //     // println!("solution: {:?}", solution);
    //     let naive_solution = naive_daily_temperatures(temperatures);
    //     // println!("naive solution: {:?}", naive_solution);
    //     assert_eq!(solution, naive_solution);
    // }
}
