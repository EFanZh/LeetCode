pub mod stack;

pub trait Solution {
    fn cal_points(ops: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["5", "2", "C", "D", "+"] as &[_], 30),
            (&["5", "-2", "4", "C", "D", "9", "+", "+"], 27),
            (&["1"], 1),
        ];

        for (ops, expected) in test_cases {
            assert_eq!(
                S::cal_points(ops.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
