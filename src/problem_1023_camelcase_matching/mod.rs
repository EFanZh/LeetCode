pub mod iterative;

pub trait Solution {
    fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"] as &[_],
                    "FB",
                ),
                &[true, false, true, true, false] as &[_],
            ),
            (
                (
                    &["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"],
                    "FoBa",
                ),
                &[true, false, true, false, false],
            ),
            (
                (
                    &["FooBar", "FooBarTest", "FootBall", "FrameBuffer", "ForceFeedBack"],
                    "FoBaT",
                ),
                &[false, true, false, false, false],
            ),
        ];

        for ((queries, pattern), expected) in test_cases {
            assert_eq!(
                S::camel_match(
                    queries.iter().copied().map(str::to_string).collect(),
                    pattern.to_string()
                ),
                expected,
            );
        }
    }
}
