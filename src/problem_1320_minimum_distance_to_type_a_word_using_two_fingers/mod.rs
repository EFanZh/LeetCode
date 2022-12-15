pub mod dijkstras_algorithm;
pub mod dynamic_programming;

pub trait Solution {
    fn minimum_distance(word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("CAKE", 3), ("HAPPY", 6), ("YEAR", 7)];

        for (word, expected) in test_cases {
            assert_eq!(S::minimum_distance(word.to_string()), expected);
        }
    }
}
