pub mod binary_search;
pub mod iterative;

pub trait Solution {
    fn preimage_size_fzf(k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 5),
            (1, 5),
            (2, 5),
            (3, 5),
            (4, 5),
            (5, 0),
            (6, 5),
            (7, 5),
            (8, 5),
            (9, 5),
            (10, 5),
            (11, 0),
            (12, 5),
            (13, 5),
            (28246, 0),
        ];

        for (k, expected) in test_cases {
            assert_eq!(S::preimage_size_fzf(k), expected);
        }
    }
}
