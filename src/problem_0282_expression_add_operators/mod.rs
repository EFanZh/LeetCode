pub mod backtracking;
pub mod backtracking_2;

pub trait Solution {
    fn add_operators(num: String, target: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("123", 6), &["1*2*3", "1+2+3"] as &[_]),
            (("232", 8), &["2*3+2", "2+3*2"]),
            (("105", 5), &["1*0+5", "10-5"]),
            (("00", 0), &["0*0", "0+0", "0-0"]),
            (("3456237490", 9191), &[]),
            (("", 5), &[]),
            (("2147483648", -2_147_483_648), &[]),
        ];

        for ((num, target), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::add_operators(num.to_string(), target)),
                expected
            );
        }
    }
}
