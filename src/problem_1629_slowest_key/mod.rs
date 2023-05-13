pub mod iterative;

pub trait Solution {
    fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[9, 29, 49, 50] as &[_], "cbcd"), 'c'),
            ((&[12, 23, 36, 46, 62], "spuda"), 'a'),
        ];

        for ((release_times, keys_pressed), expected) in test_cases {
            assert_eq!(
                S::slowest_key(release_times.to_vec(), keys_pressed.to_string()),
                expected,
            );
        }
    }
}
