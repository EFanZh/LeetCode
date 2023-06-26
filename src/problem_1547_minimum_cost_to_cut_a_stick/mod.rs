pub mod dynamic_programming;
pub mod dynamic_programming_2;

pub trait Solution {
    fn min_cost(n: i32, cuts: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((7, &[1, 3, 4, 5] as &[_]), 16), ((9, &[5, 6, 1, 4, 2]), 22)];

        for ((n, cuts), expected) in test_cases {
            assert_eq!(S::min_cost(n, cuts.to_vec()), expected);
        }
    }
}
