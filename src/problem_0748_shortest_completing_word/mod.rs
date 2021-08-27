pub mod iterative;

pub trait Solution {
    fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("1s3 PSt", &["step", "steps", "stripe", "stepple"] as &[_]), "steps"),
            (("1s3 456", &["looks", "pest", "stew", "show"]), "pest"),
            (
                (
                    "Ah71752",
                    &[
                        "suggest",
                        "letter",
                        "of",
                        "husband",
                        "easy",
                        "education",
                        "drug",
                        "prevent",
                        "writer",
                        "old",
                    ],
                ),
                "husband",
            ),
            (
                (
                    "OgEu755",
                    &[
                        "enough", "these", "play", "wide", "wonder", "box", "arrive", "money", "tax", "thus",
                    ],
                ),
                "enough",
            ),
            (
                (
                    "iMSlpe4",
                    &[
                        "claim", "consumer", "student", "camera", "public", "never", "wonder", "simple", "thought",
                        "use",
                    ],
                ),
                "simple",
            ),
        ];

        for ((license_plate, words), expected) in test_cases {
            assert_eq!(
                S::shortest_completing_word(
                    license_plate.to_string(),
                    words.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
