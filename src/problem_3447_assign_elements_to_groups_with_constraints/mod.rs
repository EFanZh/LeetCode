pub mod sieve;

pub trait Solution {
    fn assign_elements(groups: Vec<i32>, elements: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[8, 4, 3, 2, 4] as &[_], &[4, 2] as &[_]), &[0, 0, -1, 1, 0] as &[_]),
            ((&[2, 3, 5, 7], &[5, 3, 3]), &[-1, 1, 0, -1]),
            ((&[10, 21, 30, 41], &[2, 1]), &[0, 1, 0, 1]),
        ];

        for ((groups, elements), expected) in test_cases {
            assert_eq!(S::assign_elements(groups.to_vec(), elements.to_vec()), expected);
        }
    }
}
