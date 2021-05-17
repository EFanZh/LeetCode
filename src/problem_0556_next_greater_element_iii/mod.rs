pub mod next_permutation;

pub trait Solution {
    fn next_greater_element(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, -1),
            (1, -1),
            (2, -1),
            (3, -1),
            (10, -1),
            (11, -1),
            (12, 21),
            (13, 31),
            (20, -1),
            (21, -1),
            (22, -1),
            (23, 32),
            (1_999_999_999, -1),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::next_greater_element(n), expected);
        }
    }
}
