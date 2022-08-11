pub mod trie;

pub trait Solution {
    fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["mobile", "mouse", "moneypot", "monitor", "mousepad"] as &[_], "mouse"),
                &[
                    &["mobile", "moneypot", "monitor"] as &[_],
                    &["mobile", "moneypot", "monitor"],
                    &["mouse", "mousepad"],
                    &["mouse", "mousepad"],
                    &["mouse", "mousepad"],
                ] as &[&[_]],
            ),
            (
                (&["havana"], "havana"),
                &[
                    &["havana"],
                    &["havana"],
                    &["havana"],
                    &["havana"],
                    &["havana"],
                    &["havana"],
                ],
            ),
            (
                (&["bags", "baggage", "banner", "box", "cloths"], "bags"),
                &[
                    &["baggage", "bags", "banner"],
                    &["baggage", "bags", "banner"],
                    &["baggage", "bags"],
                    &["bags"],
                ],
            ),
            ((&["havana"], "tatiana"), &[&[], &[], &[], &[], &[], &[], &[]]),
        ];

        for ((products, search_word), expected) in test_cases {
            assert_eq!(
                S::suggested_products(
                    products.iter().copied().map(str::to_string).collect(),
                    search_word.to_string()
                ),
                expected
            );
        }
    }
}
