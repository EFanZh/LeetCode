pub mod floyd_warshall;

pub trait Solution {
    fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcd", "acbe", "abcced", "bcbebe", &[2, 5, 5, 1, 2, 20] as &[_]), 28),
            (("aaaa", "bbbb", "ac", "cb", &[1, 2]), 12),
            (("abcd", "abce", "a", "e", &[10000]), -1),
            (("a", "c", "aqa", "ccq", &[5, 2, 1]), 3),
        ];

        for ((source, target, original, changed, cost), expected) in test_cases {
            assert_eq!(
                S::minimum_cost(
                    source.to_string(),
                    target.to_string(),
                    original.chars().collect(),
                    changed.chars().collect(),
                    cost.to_vec(),
                ),
                expected,
            );
        }
    }
}
