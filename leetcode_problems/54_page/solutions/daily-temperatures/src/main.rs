struct Solution;

impl Solution {
    pub fn daily_temperatures(temp: Vec<i32>) -> Vec<i32> {
        match temp.len() {
            0 => vec![],
            1 => vec![0],
            _ => {
                let mut answer: Vec<i32> = vec![0; temp.len()]; 
                let mut i = 0;
                let mut j;
                while i < temp.len() - 1 {
                    (j, _) = Self::sub_interval(
                        &temp[i..],
                        &mut answer[i..]
                    );
                    i += j;
                }
                answer
            }
        }
    }

    fn sub_interval(temp: &[i32], answer: &mut[i32]) -> (usize, usize) {
        println!("recursion {:?}", temp);
        let mut i = 1;
        let mut counter = 0;
        while i < temp.len() {
            if temp[0] < temp[i] {
                answer[0] = i as i32;
                break;
            }
            if temp[i - 1] < temp[i] {
                counter = 1;
                answer[i - 1] = 1;
            } else if temp[i - 1] == temp[i] {
                let (j, c) = Self::sub_interval(
                    &temp[i..],
                    &mut answer[i..]
                );
                println!("answer = {:?}, c = {}, counter = {}", answer, c, counter);
                if c > 0 {
                    counter += c + 1;
                    answer[i - 1] = counter as i32;
                    i += j;
                } else {
                    i += j;
                    break;
                }
                // println!("updated answer = {:?}, i = {}", answer, i);
            }
            if temp[0] > temp[i] && temp.len() > 2 {
                let (j, _) = Self::sub_interval(&temp[i..], &mut answer[i..]);
                if i + j > temp.len() - 1 {
                    break;
                }
                println!("i = {}, j = {}", i, j);
                println!(
                    "temp_current = {}, tempi+j = {}",
                    temp[0], temp[i + j]
                );
                if temp[0] < temp[i + j] {
                    answer[0] = (j + 1) as i32;
                    println!("answer = {:?}", answer);
                    i += j;
                    break;
                }
                i += j;
                counter = i;
            }
            i += 1;
        }
        println!("exit recursion: i = {}, counter = {}", i, counter);
        (i, counter)
    }
}

fn main() {
    // let temperatures = vec![77,77,77,77,77,41,77,41,41,77];
    let temperatures = vec![73,74,75,71,69,72,76,73];
    let solution = Solution::daily_temperatures(temperatures);
    println!("solution: {:?}", solution);
    // assert!(solution == vec![0,0,0,0,0,1,0,2,1,0]);
    assert_eq!(solution, vec![1, 1, 4, 2, 1, 1, 0, 0]);
}
