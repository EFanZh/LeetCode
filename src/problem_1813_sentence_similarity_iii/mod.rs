pub mod iterative;

pub trait Solution {
    fn are_sentences_similar(sentence1: String, sentence2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("My name is Haley", "My Haley"), true),
            (("of", "A lot of words"), false),
            (("Eating right now", "Eating"), true),
            (
                (
                    "B",
                    "ByI BMyQIqce b bARkkMaABi vlR RLHhqjNzCN oXvyK zRXR q ff B yHS OD KkvJA P JdWksnH",
                ),
                false,
            ),
            (("C", "CB B C"), true),
            (("aa aAa", "aaA aAa"), false),
            (("A A", "A aA"), false),
            (("Are You Okay", "are you okay"), false),
        ];

        for ((sentence1, sentence2), expected) in test_cases {
            assert_eq!(
                S::are_sentences_similar(sentence1.to_string(), sentence2.to_string()),
                expected,
            );
        }
    }
}
