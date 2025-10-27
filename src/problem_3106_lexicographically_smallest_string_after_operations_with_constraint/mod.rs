pub mod greedy;

pub trait Solution {
    fn get_smallest_string(s: String, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("zbbz", 3), "aaaz"), (("xaxcd", 4), "aawcd"), (("lol", 0), "lol")];

        for ((s, k), expected) in test_cases {
            assert_eq!(S::get_smallest_string(s.to_string(), k), expected);
        }
    }
}
