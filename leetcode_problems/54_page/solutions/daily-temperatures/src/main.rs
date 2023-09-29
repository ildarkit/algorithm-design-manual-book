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
        println!("recursion {:?}", temp);
        let mut i = 1;
        let mut counter = 1;
        while i < temp.len() {
            if temp[0] < temp[i] {
                counter = std::cmp::min(i, counter);
                answer[0] = i as i32;
                break;
            }
            if i > 1 && temp[i - 1] < temp[i] {
                counter = 1;
                answer[i - 1] = counter as i32;
            } else if temp[i - 1] == temp[i] {
                let c = Self::sub_interval(
                    &temp[i..],
                    &mut answer[i..]
                );
                println!("answer = {:?}, c = {}, counter = {}", answer, c, counter);
                if c > 0 {
                    counter += c;
                    answer[i - 1] = counter as i32;
                    i += c;
                } else {
                    counter = 0;
                    break;
                }
                // println!("updated answer = {:?}, i = {}", answer, i);
            }
            if temp[0] > temp[i] && i < temp.len() - 1 {
                let c = Self::sub_interval(&temp[i..], &mut answer[i..]);
                println!("i = {}, c = {}", i, c);
                println!(
                    "temp_current = {}, tempi+c = {}",
                    temp[0], temp[i + c]
                );
                i += c;
                if temp[0] < temp[i] {
                    counter += c;
                    answer[0] = counter as i32;
                    println!("answer = {:?}", answer);
                    break;
                } else if temp[0] == temp[i] {
                    counter = 0;
                } else {
                    counter = i;
                }
            }
            i += 1;
        }
        println!("exit recursion: i = {}, counter = {}", i, counter);
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
    let temperatures = vec![64, 35, 74, 46, 74, 52, 52, 81, 98, 71, 69];
    println!("temperatures = {:?}", temperatures);
    let solution = Solution::daily_temperatures(temperatures);
    println!("solution: {:?}", solution);
    // assert_eq!(solution, vec![1, 2, 1, 0, 1, 0, 3, 1, 1, 0, 0]);
    // assert_eq!(solution, vec![0, 0, 0, 0, 0, 1, 0, 2, 1, 0]);
    // assert_eq!(solution, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    assert_eq!(solution, vec![2, 1, 5, 1, 3, 2, 1, 1, 0, 0, 0]);

    let dist = Uniform::new_inclusive(30, 100);
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let temperatures: Vec<_> = (0..11).map(|_| dist.sample(&mut rng)).collect();
        println!("temperatures = {:?}", temperatures);
        let solution = Solution::daily_temperatures(temperatures.clone());
        println!("solution: {:?}", solution);
        let naive_solution = naive_daily_temperatures(temperatures);
        println!("naive solution: {:?}", naive_solution);
        assert_eq!(solution, naive_solution);
    }
}
