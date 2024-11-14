pub mod matrix_multiplication;

pub trait Solution {
    fn count_house_placements(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, 1), (1, 4), (2, 9), (3, 25), (4, 64), (5, 169), (1000, 500_478_595)];

        for (n, expected) in test_cases {
            assert_eq!(S::count_house_placements(n), expected);
        }
    }
}
