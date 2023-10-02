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
                }
                answer
            }
        }
    }

    fn sub_interval(temp: &[i32], answer: &mut[i32]) -> usize {
        let mut i = 1;
        let mut counter = 1;
        let mut c;
        while i < temp.len() {
            if i < temp.len() - 1 && temp[i - 1] == temp[i] {
                c = Self::sub_interval(
                    &temp[i..],
                    &mut answer[i..]
                );
                if answer[i] > 0 && answer[i - 1] == 0 {
                    answer[i - 1] += answer[i] + 1;
                }
                i += c;
                if counter == 1 {
                    counter += c;
                }
            }

            if i < temp.len() - 1 && temp[0] > temp[i] {
                c = Self::sub_interval(&temp[i..], &mut answer[i..]);
                i += c;
                if counter == 1 {
                    counter += c;
                }
            }

            if i < temp.len() && temp[0] < temp[i] {
                if answer[0] == 0 {
                    answer[0] = i as i32;
                }
                break;
            }
            i += 1;
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
    // // let temperatures = vec![77,77,77,77,77,41,77,41,41,77];
    // // let temperatures = vec![73,74,75,71,69,72,76,73];
    // // let temperatures = vec![50, 72, 41, 90, 31, 81, 40, 32, 33, 81, 47];
    // // let temperatures = vec![64, 35, 74, 46, 74, 52, 52, 81, 98, 71, 69];
    // // let temperatures = vec![33, 74, 94, 56, 85, 97, 72, 81, 50, 85, 42];
    // // let temperatures = vec![86, 65, 85, 90, 65, 32, 83, 83, 81, 91, 100];
    // // let temperatures = vec![31, 100, 91, 50, 83, 87, 76, 43, 31, 31, 30];
    // // let temperatures = vec![95, 86, 74, 74, 30, 71, 76, 84, 67, 38, 42];
    // // let temperatures = vec![78, 93, 42, 62, 62, 83, 83, 92, 99, 58];
    // let temperatures = vec![88, 33, 37, 74, 37, 49, 39, 53, 43, 43];
    // println!("temperatures = {:?}", temperatures);
    // let solution = Solution::daily_temperatures(temperatures.clone());
    // println!("solution: {:?}", solution);
    // let naive_solution = naive_daily_temperatures(temperatures);
    // println!("naive solution: {:?}", naive_solution);
    // // assert_eq!(solution, vec![0, 0, 0, 0, 0, 1, 0, 2, 1, 0]);
    // // assert_eq!(solution, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    // // assert_eq!(solution, vec![1, 2, 1, 0, 1, 0, 3, 1, 1, 0, 0]);
    // // assert_eq!(solution, vec![2, 1, 5, 1, 3, 2, 1, 1, 0, 0, 0]);
    // // assert_eq!(solution, vec![1, 1, 3, 1, 1, 0, 1, 2, 1, 0, 0]);
    // // assert_eq!(solution, vec![3, 1, 1, 6, 2, 1, 3, 2, 1, 1, 0]);
    // // assert_eq!(solution, vec![1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0]);
    // // assert_eq!(solution, vec![0, 0, 4, 3, 1, 1, 1, 0, 0, 1, 0]);
    // // assert_eq!(solution, vec![1, 7, 1, 2, 1, 2, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![0, 1, 1, 0, 1, 2, 1, 0, 0, 0]);

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
