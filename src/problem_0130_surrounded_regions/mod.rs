pub mod bfs;

pub trait Solution {
    fn solve(board: &mut Vec<Vec<char>>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &["XXXX", "XOOX", "XXOX", "XOXX"] as &[&str],
            &["XXXX", "XXXX", "XXXX", "XOXX"] as &[_],
        )];

        for (nums, expected) in test_cases.iter().copied() {
            let mut nums = nums.iter().map(|row| row.chars().collect()).collect();

            S::solve(&mut nums);

            assert_eq!(
                nums.into_iter()
                    .map(|row| row.into_iter().collect::<String>())
                    .collect::<Box<_>>()
                    .as_ref(),
                expected
            );
        }
    }
}
