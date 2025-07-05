pub mod vec_deque;

pub trait Solution {
    fn final_string(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("string", "rtsng"), ("poiinter", "ponter")];

        for (s, expected) in test_cases {
            assert_eq!(S::final_string(s.to_string()), expected);
        }
    }
}
