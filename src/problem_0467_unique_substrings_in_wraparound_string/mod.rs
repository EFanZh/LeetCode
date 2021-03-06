pub mod brute_force;

pub trait Solution {
    fn find_substring_in_wrapround_string(p: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("a", 1), ("cac", 2), ("zab", 6), ("", 0)];

        for (p, expected) in test_cases.iter().copied() {
            assert_eq!(S::find_substring_in_wrapround_string(p.to_string()), expected);
        }
    }
}
