pub mod connected_components;

pub trait Solution {
    fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("dcab", &[[0, 3], [1, 2]] as &[_]), "bacd"),
            (("dcab", &[[0, 3], [1, 2], [0, 2]]), "abcd"),
            (("cba", &[[0, 1], [1, 2]]), "abc"),
            (("dcab", &[]), "dcab"),
        ];

        for ((s, pairs), expected) in test_cases {
            assert_eq!(
                S::smallest_string_with_swaps(s.to_string(), pairs.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
