pub mod hash_map;

pub trait Solution {
    fn kth_distinct(arr: Vec<String>, k: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["d", "b", "c", "b", "c", "a"] as &[_], 2), "a"),
            ((&["aaa", "aa", "a"], 1), "aaa"),
            ((&["a", "b", "a"], 3), ""),
        ];

        for ((arr, k), expected) in test_cases {
            assert_eq!(
                S::kth_distinct(arr.iter().copied().map(str::to_string).collect(), k),
                expected,
            );
        }
    }
}
