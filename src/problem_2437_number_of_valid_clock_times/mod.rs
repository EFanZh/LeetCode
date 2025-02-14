pub mod mathematical;

pub trait Solution {
    fn count_time(time: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("?5:00", 2),
            ("0?:0?", 100),
            ("??:??", 1440),
            ("07:?3", 6),
            ("2?:??", 240),
            ("?2:16", 3),
            ("?4:22", 2),
        ];

        for (time, expected) in test_cases {
            assert_eq!(S::count_time(time.to_string()), expected);
        }
    }
}
