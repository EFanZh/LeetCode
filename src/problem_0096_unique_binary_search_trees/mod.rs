pub mod catalan_number;
pub mod catalan_number_2;
pub mod dynamic_programming;

pub trait Solution {
    fn num_trees(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 5),
            (4, 14),
            (5, 42),
            (6, 132),
            (7, 429),
            (8, 1_430),
            (9, 4_862),
            (10, 16_796),
            (11, 58_786),
            (12, 208_012),
            (13, 742_900),
            (14, 2_674_440),
            (15, 9_694_845),
            (16, 35_357_670),
            (17, 129_644_790),
            (18, 477_638_700),
            (19, 1_767_263_190),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::num_trees(n), expected);
        }
    }
}
