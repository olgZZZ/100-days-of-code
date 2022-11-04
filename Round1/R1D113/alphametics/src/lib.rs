use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut values = parse(input);
    values.sort();
    let mut ans = vec![0; values.len()];

    return find(0, &mut ans, &(0..10).collect(), &values);
}

fn parse(input: &str) -> Vec<(char, i64, bool)> {
    let mut ans = HashMap::new();
    let mut leading = HashSet::new();
    let mut prev = ' ';
    let mut value = -1;

    for ch in input.chars().rev() {
        match ch {
            'A'..='Z' => {
                *ans.entry(ch).or_insert(0) += value;
                value *= 10;
                prev = ch;
            }

            _ => {
                value = 1;
                leading.insert(prev);
            }
        }
    }

    leading.insert(prev);

    ans.iter()
        .map(|(&k, &v)| (k, v, leading.contains(&k)))
        .collect()
}

fn find(
    count: usize,
    ans: &mut Vec<u8>,
    digits: &Vec<u8>,
    values: &Vec<(char, i64, bool)>,
) -> Option<HashMap<char, u8>> {
    if count == values.len() {
        return if cal(ans, values) {
            Some(
                ans.iter()
                    .zip(values.iter())
                    .map(|(&digit, &(ch, _, _))| (ch, digit))
                    .collect(),
            )
        } else {
            None
        };
    }

    for (i, &digit) in digits.iter().enumerate() {
        if digit == 0 && values[count].2 {
            continue;
        }

        ans[count] = digit as u8;
        let mut digits2 = digits.clone();
        digits2.remove(i);

        if let Some(solution) = find(count + 1, ans, &digits2, values) {
            return Some(solution);
        }
    }

    None
}

fn cal(ans: &Vec<u8>, values: &Vec<(char, i64, bool)>) -> bool {
    let mut total = 0;

    for (i, &digit) in ans.iter().enumerate() {
        total += digit as i64 * values[i].1;
    }

    total == 0
}
