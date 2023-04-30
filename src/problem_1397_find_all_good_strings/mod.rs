pub mod memoized_dynamic_programming_with_kmp;

pub trait Solution {
    fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, "aa", "da", "b"), 51),
            ((8, "leetcode", "leetgoes", "leet"), 0),
            ((2, "gx", "gz", "x"), 2),
            ((3, "szc", "zyi", "p"), 4357),
            ((8, "pzdanyao", "wgpmtywi", "sdka"), 500_543_753),
            ((7, "sbhznfr", "zpxmvlw", "vxlcovq"), 335_838_379),
        ];

        for ((n, s1, s2, evil), expected) in test_cases {
            assert_eq!(
                S::find_good_strings(n, s1.to_string(), s2.to_string(), evil.to_string()),
                expected,
            );
        }
    }
}
