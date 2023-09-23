pub mod iterative;

pub trait Solution {
    fn sort_sentence(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("is2 sentence4 This1 a3", "This is a sentence"),
            ("Myself2 Me1 I4 and3", "Me Myself and I"),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::sort_sentence(s.to_string()), expected);
        }
    }
}
