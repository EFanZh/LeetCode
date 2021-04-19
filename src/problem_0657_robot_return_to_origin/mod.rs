pub mod iterative;

pub trait Solution {
    fn judge_circle(moves: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("UD", true), ("LL", false), ("RRDD", false), ("LDRRLRUULR", false)];

        for (moves, expected) in test_cases.iter().copied() {
            assert_eq!(S::judge_circle(moves.to_string()), expected);
        }
    }
}
