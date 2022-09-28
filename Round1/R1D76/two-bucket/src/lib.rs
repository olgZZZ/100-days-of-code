use std::{cmp, collections::HashSet};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,

    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,

    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

fn swap<T>(x: (T, T)) -> (T, T) {
    return (x.1, x.0);
}

fn spill(cap: u8, from: u8, to: u8) -> (u8, u8) {
    let max = cap - to;
    let actual = cmp::min(from, max);

    (from - actual, to + actual)
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut moves: u8 = 1;

    let mut combinations: HashSet<(u8, u8)> =
        vec![(capacity_1, 0), (0, capacity_2)].into_iter().collect();

    let mut last = match start_bucket {
        Bucket::One => vec![(capacity_1, 0)],
        Bucket::Two => vec![(0, capacity_2)],
    };

    while last.clone().len() > 0 {
        let current = last.clone();

        last = Vec::new();

        for (one, two) in current.into_iter() {
            if (one == goal) || (two == goal) {
                let bucket_stats = BucketStats {
                    moves,

                    goal_bucket: match one == goal {
                        true => Bucket::One,
                        _ => Bucket::Two,
                    },

                    other_bucket: match one == goal {
                        true => two,
                        _ => one,
                    },
                };

                return Some(bucket_stats);
            }

            vec![
                spill(capacity_2, one, two),
                swap(spill(capacity_1, two, one)),
                (0, two),
                (one, 0),
                (capacity_1, two),
                (one, capacity_2),
            ]
            .into_iter()
            .for_each(|combination| match combinations.contains(&combination) {
                false => {
                    combinations.insert(combination.clone());

                    last.push(combination.clone());
                }

                _ => (),
            });
        }

        moves += 1;
    }

    None
}
