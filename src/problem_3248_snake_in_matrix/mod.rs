pub mod iterative;

pub trait Solution {
    fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((2, &["RIGHT", "DOWN"] as &[_]), 3), ((3, &["DOWN", "RIGHT", "UP"]), 1)];

        for ((n, commands), expected) in test_cases {
            assert_eq!(
                S::final_position_of_snake(n, commands.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
