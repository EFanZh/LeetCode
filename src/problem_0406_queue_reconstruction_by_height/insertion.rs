pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;

        people.sort_unstable_by_key(|p| {
            let [h, k] = p.as_slice().try_into().unwrap();

            (Reverse(h), k)
        });

        let mut i = 0;

        while let Some(p) = people.get(i) {
            let k = p[1] as _;

            people[k..=i].rotate_right(1);

            i += 1;
        }

        people
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
