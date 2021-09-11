pub mod brute_force;

pub trait Solution {
    fn num_jewels_in_stones(j: String, s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("aA", "aAAbbbb"), 3), (("z", "ZZ"), 0)];

        for ((j, s), expected) in test_cases {
            assert_eq!(S::num_jewels_in_stones(j.to_string(), s.to_string()), expected);
        }
    }
}
