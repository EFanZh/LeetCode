pub mod greedy;

pub trait Solution {
    fn find_latest_time(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("1?:?4", "11:54"), ("0?:5?", "09:59")];

        for (s, expected) in test_cases {
            assert_eq!(S::find_latest_time(s.to_string()), expected);
        }
    }
}
