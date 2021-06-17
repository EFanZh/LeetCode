pub mod iterative;

pub trait Solution {
    fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[
                        ["phone", "blue", "pixel"],
                        ["computer", "silver", "lenovo"],
                        ["phone", "gold", "iphone"],
                    ] as &[_],
                    "color",
                    "silver",
                ),
                1,
            ),
            (
                (
                    &[
                        ["phone", "blue", "pixel"],
                        ["computer", "silver", "phone"],
                        ["phone", "gold", "iphone"],
                    ],
                    "type",
                    "phone",
                ),
                2,
            ),
            (
                (
                    &[
                        ["phone", "blue", "pixel"],
                        ["computer", "silver", "phone"],
                        ["phone", "gold", "iphone"],
                    ],
                    "name",
                    "phone",
                ),
                1,
            ),
        ];

        for ((items, rule_key, rule_value), expected) in test_cases {
            assert_eq!(
                S::count_matches(
                    items
                        .iter()
                        .map(|item| item.iter().copied().map(str::to_string).collect())
                        .collect(),
                    rule_key.to_string(),
                    rule_value.to_string()
                ),
                expected
            );
        }
    }
}
