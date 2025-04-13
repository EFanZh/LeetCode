pub mod mathematical;

pub trait Solution {
    fn monkey_move(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(3, 6), (4, 14), (5, 30)];

        for (n, expected) in test_cases {
            assert_eq!(S::monkey_move(n), expected);
        }
    }
}
