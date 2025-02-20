pub mod mathematical;

pub trait Solution {
    fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((["01:15", "02:00"], ["02:00", "03:00"]), true),
            ((["01:00", "02:00"], ["01:20", "03:00"]), true),
            ((["10:00", "11:00"], ["14:00", "15:00"]), false),
        ];

        for ((event1, event2), expected) in test_cases {
            assert_eq!(
                S::have_conflict(
                    event1.iter().copied().map(str::to_string).collect(),
                    event2.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
