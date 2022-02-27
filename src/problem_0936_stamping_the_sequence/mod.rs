pub mod greedy;

pub trait Solution {
    fn moves_to_stamp(stamp: String, target: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abc", "ababc"), true),
            (("abca", "aabcaca"), true),
            (("df", "dfdff"), true),
            (("aye", "eyeye"), false),
        ];

        for ((stamp, target), expected) in test_cases {
            let result = S::moves_to_stamp(stamp.to_string(), target.to_string());

            if expected {
                let n = stamp.len();
                let mut buffer = vec![0; target.len()];

                for start in result {
                    let start = start as usize;

                    buffer[start..start + n].copy_from_slice(stamp.as_bytes());
                }

                assert_eq!(buffer, target.as_bytes());
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
