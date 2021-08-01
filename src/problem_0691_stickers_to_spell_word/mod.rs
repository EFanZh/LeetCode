pub mod bfs;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn min_stickers(stickers: Vec<String>, target: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["with", "example", "science"] as &[_], "thehat"), 3),
            ((&["notice", "possible"], "basicbasic"), -1),
        ];

        for ((stickers, target), expected) in test_cases {
            assert_eq!(
                S::min_stickers(
                    stickers.iter().copied().map(str::to_string).collect(),
                    target.to_string()
                ),
                expected
            );
        }
    }
}
