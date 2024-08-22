pub mod union_find;

pub trait Solution {
    fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (6, &[[1, 2, 5], [2, 3, 8], [1, 5, 10]] as &[_], 1),
                &[0, 1, 2, 3, 5] as &[_],
            ),
            ((4, &[[3, 1, 3], [1, 2, 2], [0, 3, 3]], 3), &[0, 1, 3]),
            ((5, &[[3, 4, 2], [1, 2, 1], [2, 3, 1]], 1), &[0, 1, 2, 3, 4]),
            ((6, &[[0, 2, 1], [1, 3, 1], [4, 5, 1]], 1), &[0, 1, 2, 3]),
            ((11, &[[5, 1, 4], [0, 4, 18]], 1), &[0, 1, 4, 5]),
        ];

        for ((n, meetings, first_person), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_all_people(
                    n,
                    meetings.iter().map(Vec::from).collect(),
                    first_person,
                )),
                expected,
            );
        }
    }
}
