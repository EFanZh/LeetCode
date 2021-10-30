pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::convert::TryInto;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;

        people.sort_unstable_by_key(|p| {
            let [h, k]: [_; 2] = p.as_slice().try_into().unwrap();

            (h, Reverse(k))
        });

        let mut result = vec![Vec::new(); people.len()];

        for p in people {
            let k = p[1] as _;

            *result.iter_mut().filter(|v| v.is_empty()).nth(k).unwrap() = p;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::reconstruct_queue(people)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
