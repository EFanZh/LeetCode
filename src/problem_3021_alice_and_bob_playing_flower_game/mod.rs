pub mod mathematical;

pub trait Solution {
    fn flower_game(n: i32, m: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((3, 2), 3), ((1, 1), 0)];

        for ((n, m), expected) in test_cases {
            assert_eq!(S::flower_game(n, m), expected);
        }
    }
}
