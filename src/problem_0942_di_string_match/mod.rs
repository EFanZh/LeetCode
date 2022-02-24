pub mod dynamic_programming;

pub trait Solution {
    fn di_string_match(s: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = ["IDID", "III", "DDI"];

        for s in test_cases {
            let mut result = S::di_string_match(s.to_string());

            assert_eq!(result.len(), s.len() + 1);

            for ((lhs, rhs), expected) in result.iter().zip(&result[1..]).zip(s.bytes()) {
                if expected == b'D' {
                    assert!(lhs > rhs);
                } else {
                    assert!(lhs < rhs);
                }
            }

            result.sort_unstable();

            for (i, num) in (0..).zip(result) {
                assert_eq!(i, num);
            }
        }
    }
}
