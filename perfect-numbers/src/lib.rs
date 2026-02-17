#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    use crate::Classification::*;
    let mut vec: Vec<u64> = Vec::new();
    vec.push(1);
    for i in 1..num {
        for k in i..num {
            if i * k == num {
                if i != k {
                    vec.push(i);
                    vec.push(k);
                } else if i == k {
                    vec.push(i);
                }
            }
        }
    }
    println!(" =====>>> Input is {} , {:?} .", num, vec);

    let sum: u64 = vec.iter().sum();
    println!(" =====>>> Sum is {} .", sum);

    if sum == num && vec.len() == 1 {
        return Some(Deficient);
    }

    if sum == num {
        return Some(Perfect);
    } else if sum > num {
        return Some(Abundant);
    } else if sum < num {
        return Some(Deficient);
    } else {
        return None;
    }

    //Some(Perfect)
}
