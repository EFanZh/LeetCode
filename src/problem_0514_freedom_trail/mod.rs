pub mod dynamic_programming;

pub trait Solution {
    fn find_rotate_steps(ring: String, key: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("godding", "gd"), 4), (("godding", "godding"), 13)];

        for ((ring, key), expected) in test_cases {
            assert_eq!(S::find_rotate_steps(ring.to_string(), key.to_string()), expected);
        }
    }
}
