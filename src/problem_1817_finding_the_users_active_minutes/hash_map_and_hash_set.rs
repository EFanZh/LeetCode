pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut users = HashMap::<i32, HashSet<_>>::new();

        for log in logs {
            let [id, time]: [_; 2] = log.try_into().ok().unwrap();

            match users.entry(id) {
                Entry::Occupied(entry) => {
                    entry.into_mut().insert(time);
                }
                Entry::Vacant(entry) => {
                    entry.insert(HashSet::from([time]));
                }
            }
        }

        let mut result = vec![0; k as u32 as usize];

        for (_, times) in users {
            result[times.len() - 1] += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        Self::finding_users_active_minutes(logs, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
