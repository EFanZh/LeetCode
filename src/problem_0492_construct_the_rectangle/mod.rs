pub mod iterative;

pub trait Solution {
    fn construct_rectangle(area: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(4, [2, 2]), (37, [37, 1]), (122_122, [427, 286])];

        for (area, expected) in test_cases.iter().copied() {
            assert_eq!(S::construct_rectangle(area), expected);
        }
    }
}
