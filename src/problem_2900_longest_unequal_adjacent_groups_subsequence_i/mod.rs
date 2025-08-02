pub mod iterative;

pub trait Solution {
    fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["e", "a", "b"] as &[_], &[0, 0, 1] as &[_]), 2),
            ((&["a", "b", "c", "d"], &[1, 0, 1, 1]), 3),
        ];

        for ((words, groups), expected) in test_cases {
            let result =
                S::get_longest_subsequence(words.iter().copied().map(str::to_string).collect(), groups.to_vec());

            assert_eq!(result.len(), expected);

            let mut iter = words.iter().copied();

            assert!(result.iter().all(|word| iter.any(|x| x == word)));

            let mut prev = 2;

            for word in &result {
                let index = words.iter().position(|x| x == word).unwrap();
                let group = groups[index];

                assert_ne!(group, prev);

                prev = group;
            }
        }
    }
}
