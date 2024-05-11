pub mod mathematical;

pub trait Solution {
    fn sum_game(num: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("5023", false), ("25??", true), ("?3295???", false)];

        for (num, expected) in test_cases {
            assert_eq!(S::sum_game(num.to_string()), expected);
        }
    }
}
