pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        assert_eq!(words.len(), groups.len());

        let mut words = words;
        let mut iter = groups.iter().copied();
        let mut prev = 2;

        words.retain(|_| {
            let group = iter.next().unwrap();
            let retain = group != prev;

            prev = group;

            retain
        });

        words
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        Self::get_longest_subsequence(words, groups)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
