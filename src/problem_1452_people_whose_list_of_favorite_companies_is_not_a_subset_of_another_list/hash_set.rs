pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut ids = HashMap::new();

        let mut favorite_companies = (0..)
            .zip(favorite_companies)
            .map(|(i, list)| {
                (
                    i,
                    list.into_iter()
                        .map(|company| {
                            let candidate = ids.len();

                            *ids.entry(company).or_insert(candidate as u16)
                        })
                        .collect::<HashSet<_>>(),
                )
            })
            .collect::<Vec<_>>();

        favorite_companies.sort_unstable_by_key(|(_, list)| Reverse(list.len()));

        let mut result = Vec::new();

        for (i, list) in &favorite_companies {
            let mut iter = favorite_companies.iter();

            loop {
                if let Some((_, prev)) = iter.next().filter(|(_, prev)| list.len() < prev.len()) {
                    if list.iter().all(|company| prev.contains(company)) {
                        break;
                    }
                } else {
                    result.push(*i);

                    break;
                }
            }
        }

        result.sort_unstable();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        Self::people_indexes(favorite_companies)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
