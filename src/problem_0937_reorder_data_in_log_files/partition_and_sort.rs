pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn is_letter_log(log: &str) -> bool {
        let log = log.as_bytes();
        let i = log.iter().position(|&c| c == b' ').unwrap();

        log[i + 1..].iter().all(|c| matches!(c, b' ' | b'a'..=b'z'))
    }

    fn key(log: &str) -> (&[u8], &[u8]) {
        let log = log.as_bytes();
        let i = log.iter().position(|&c| c == b' ').unwrap();

        (&log[i + 1..], &log[..i])
    }

    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut logs = logs;

        if let Some(mut i) = logs.iter().rposition(|log| Self::is_letter_log(log)) {
            let mut j = i.wrapping_sub(1);

            while let Some(log) = logs.get(j) {
                if !Self::is_letter_log(log) {
                    logs.swap(i, j);
                    i = i.wrapping_sub(1);
                }

                j = j.wrapping_sub(1);
            }

            logs[..i.wrapping_add(1)].sort_unstable_by(|lhs, rhs| Self::key(lhs).cmp(&Self::key(rhs)));
        }

        logs
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        Self::reorder_log_files(logs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
