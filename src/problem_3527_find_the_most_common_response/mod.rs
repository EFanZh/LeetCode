pub mod hash_map_and_hash_set;

pub trait Solution {
    fn find_common_response(responses: Vec<Vec<String>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    &["good", "ok", "good", "ok"] as &[_],
                    &["ok", "bad", "good", "ok", "ok"],
                    &["good"],
                    &["bad"],
                ] as &[&[_]],
                "good",
            ),
            (
                &[
                    &["good", "ok", "good"],
                    &["ok", "bad"],
                    &["bad", "notsure"],
                    &["great", "good"],
                ],
                "bad",
            ),
            (&[&["gzdk", "l", "l", "opo", "ny"]], "gzdk"),
        ];

        for (responses, expected) in test_cases {
            assert_eq!(
                S::find_common_response(
                    responses
                        .iter()
                        .map(|responses| responses.iter().copied().map(str::to_string).collect())
                        .collect()
                ),
                expected,
            );
        }
    }
}
