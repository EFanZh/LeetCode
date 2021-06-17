pub mod backtracking;

pub trait Solution {
    fn read_binary_watch(num: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            1,
            &[
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
            ] as &[_],
        )];

        for (num, expected) in test_cases {
            assert_eq!(test_utilities::unstable_sorted(S::read_binary_watch(num)), expected);
        }
    }
}
