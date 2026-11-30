pub mod iterative;

pub trait Solution {
    fn assign_umbrella(rain: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[1, 1, 0, 1, 1] as &[_],
                &["Alice", "Bob", "None", "Alice", "Bob"] as &[_],
            ),
            (&[0, 1, 0, 1], &["None", "Bob", "None", "Bob"]),
        ];

        for (rain, expected) in test_cases {
            assert_eq!(S::assign_umbrella(rain.to_vec()), expected);
        }
    }
}
