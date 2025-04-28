pub mod mathematical;

pub trait Solution {
    fn colored_cells(num: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(1, 1), (2, 5), (3, 13), (4, 25)];

        for (num, expected) in test_cases {
            assert_eq!(S::colored_cells(num), expected);
        }
    }
}
