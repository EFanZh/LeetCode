pub mod binary_search;
pub mod newtons_method;

pub trait Solution {
    fn my_sqrt(x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 2),
            (6, 2),
            (7, 2),
            (8, 2),
            (9, 3),
            (10, 3),
        ];

        for (x, expected) in test_cases.iter().copied() {
            assert_eq!(S::my_sqrt(x), expected);
        }
    }
}
