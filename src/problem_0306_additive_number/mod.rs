pub mod brute_force;

pub trait Solution {
    fn is_additive_number(num: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("112358", true),
            ("199100199", true),
            ("1203", false),
            ("0235813", false),
            ("0", false),
            ("000", true),
            ("111", false),
            ("199111992", true),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::is_additive_number(num.to_string()), expected);
        }
    }
}
