pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::VecDeque;

enum ProcessResult {
    NoMatch,
    Matched,
    Cleared,
}

impl Solution {
    fn process_window(window: &mut [u8], stamp: &[u8]) -> ProcessResult {
        let mut has_non_zero = false;

        for (&actual, &expected) in window.iter().zip(stamp) {
            if actual != 0 {
                if actual == expected {
                    has_non_zero = true;
                } else {
                    return ProcessResult::NoMatch;
                }
            }
        }

        if has_non_zero {
            window.fill(0);

            ProcessResult::Matched
        } else {
            ProcessResult::Cleared
        }
    }

    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp = stamp.into_bytes();
        let mut target = target.into_bytes();
        let n = stamp.len();
        let mut result = Vec::new();
        let mut queue = (0..=target.len() - n).collect::<VecDeque<_>>();

        loop {
            let mut has_progress = false;

            for _ in 0..queue.len() {
                let start = queue.pop_front().unwrap();

                match Self::process_window(&mut target[start..start + n], &stamp) {
                    ProcessResult::NoMatch => queue.push_back(start),
                    ProcessResult::Matched => {
                        has_progress = true;

                        result.push(start as _);
                    }
                    ProcessResult::Cleared => has_progress = true,
                }
            }

            if !has_progress {
                result.clear();

                return result;
            }

            if queue.is_empty() {
                break;
            }
        }

        result.reverse();

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        Self::moves_to_stamp(stamp, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
