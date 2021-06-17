pub mod dynamic_programming;
pub mod memoized_recursive;

pub trait Solution {
    fn is_scramble(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("great", "rgeat"), true),
            (("great", "rgtae"), true),
            (("abcde", "caebd"), false),
            (("abcdefghijklmnopq", "efghijklmnopqcadb"), false),
            (("a", "a"), true),
            (
                (
                    "bcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcdebcde",
                    "cebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebdcebd",
                ),
                false,
            ),
        ];

        for ((s1, s2), expected) in test_cases {
            assert_eq!(S::is_scramble(s1.to_string(), s2.to_string()), expected);
        }
    }
}
