pub mod brute_force;

pub trait Solution {
    fn maximum_time(time: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("2?:?0", "23:50"),
            ("0?:3?", "09:39"),
            ("1?:22", "19:22"),
            ("?0:12", "20:12"),
            ("?4:32", "14:32"),
            ("??:01", "23:01"),
        ];

        for (s, expected) in test_cases.iter().copied() {
            assert!(S::maximum_time(s.to_string()) == expected);
        }
    }
}
