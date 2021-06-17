pub mod binary_search_tree;
pub mod fenwick_tree;
pub mod insertion;
pub mod insertion_2;

pub trait Solution {
    fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[7, 0], [4, 4], [7, 1], [5, 0], [6, 1], [5, 2]] as &[_],
                &[[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]] as &[_],
            ),
            (
                &[[6, 0], [5, 0], [4, 0], [3, 2], [2, 2], [1, 4]],
                &[[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]],
            ),
            (&[], &[]),
            (&[[2, 0]], &[[2, 0]]),
        ];

        for (people, expected) in test_cases {
            assert_eq!(
                S::reconstruct_queue(people.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
