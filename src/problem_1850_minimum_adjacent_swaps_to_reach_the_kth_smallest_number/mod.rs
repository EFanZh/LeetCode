pub mod brute_force;

pub trait Solution {
    fn get_min_swaps(num: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("5489355142", 4), 2), (("11112", 4), 4), (("00123", 1), 1)];

        for ((num, k), expected) in test_cases {
            assert_eq!(S::get_min_swaps(num.to_string(), k), expected);
        }
    }
}
