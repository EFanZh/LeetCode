pub mod brute_force;

pub trait Solution {
    fn reordered_power_of2(n: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, true),
            (10, false),
            (16, true),
            (24, false),
            (46, true),
            (821, true),
            (1_420, true),
            (14_683, true),
            (121_073, true),
            (4_567_801, true),
            (66_771_127, true),
            (778_112_234, true),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::reordered_power_of2(n), expected);
        }
    }
}
