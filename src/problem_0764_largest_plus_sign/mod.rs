pub mod dynamic_programming;

pub trait Solution {
    fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((5, &[[4, 2]] as &[_]), 2), ((1, &[[0, 0]]), 0), ((500, &[]), 250)];

        for ((n, mines), expected) in test_cases {
            assert_eq!(
                S::order_of_largest_plus_sign(n, mines.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
