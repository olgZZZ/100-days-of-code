#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }
    let sum = factions(num).iter().fold(0, |acc, i| acc + i);
    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

fn factions(num: u64) -> Vec<u64> {
    (1..)
        .take_while(|i| i < &num)
        .filter(|i| num % i == 0)
        .collect()
}

#[test]
fn test_factions() {
    println!("{:?}", factions(6));
    assert!(factions(6) == vec![1, 2, 3])
}

#[test]
fn test_factions2() {
    println!("{:?}", factions(28));
    assert!(factions(28) == vec![1, 2, 4, 7, 14])
}