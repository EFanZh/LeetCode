pub struct Solution {}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

impl Solution {
    fn combination_sum2_helper<'a, I: Iterator<Item = (&'a i32, &'a i32)> + Clone>(
        mut counts: I,
        mut target: i32,
        used: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(used.clone())
        } else if let Some((&num, &count)) = counts.next() {
            let original_size = used.len();

            Self::combination_sum2_helper(counts.clone(), target, used, result);

            for _ in 0..count {
                if target >= num {
                    used.push(num);
                    target -= num;

                    Self::combination_sum2_helper(counts.clone(), target, used, result);
                } else {
                    break;
                }
            }

            used.truncate(original_size);
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut counts = HashMap::new();

        for candidate in candidates {
            match counts.entry(candidate) {
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
                Entry::Occupied(entry) => *entry.into_mut() += 1,
            }
        }

        let mut result = Vec::new();

        Self::combination_sum2_helper(counts.iter(), target, &mut Vec::new(), &mut result);

        result
    }
}

impl super::Solution for Solution {
    fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::combination_sum2(candidates, target)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
