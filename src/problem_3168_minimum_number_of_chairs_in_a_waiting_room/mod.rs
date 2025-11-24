pub mod brute_force;

pub trait Solution {
    fn minimum_chairs(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("EEEEEEE", 7), ("ELELEEL", 2)];

        for (s, expected) in test_cases {
            assert_eq!(S::minimum_chairs(s.to_string()), expected);
        }
    }
}
