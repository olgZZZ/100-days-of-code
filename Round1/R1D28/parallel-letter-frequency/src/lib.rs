use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], wc: usize) -> HashMap<char, usize> {
    let mut pool = vec![];
    let mut ans: HashMap<char, usize> = HashMap::new();

    // split input into wc chunks
    for lines in input.chunks(input.len() / wc + 1) {
        let s = lines.join("");

        let worker = thread::spawn(move || {
            let mut mp: HashMap<char, usize> = HashMap::new();

            for c in s
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *mp.entry(c).or_default() += 1;
            }

            mp
        });

        pool.push(worker);
    }

    for w in pool {
        let resp = w.join().unwrap();

        for (&k, v) in resp.iter() {
            *ans.entry(k).or_default() += v;
        }
    }

    ans
}
