pub mod brute_force;
pub mod two_pointers;

pub trait Solution {
    fn judge_square_sum(c: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, true),
            (2, true),
            (3, false),
            (4, true),
            (5, true),
            (12_132_321, false),
        ];

        for (c, expected) in test_cases {
            assert_eq!(S::judge_square_sum(c), expected);
        }
    }
}
