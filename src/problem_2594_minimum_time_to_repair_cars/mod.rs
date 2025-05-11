pub mod binary_search;

pub trait Solution {
    fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 3, 1] as &[_], 10), 16),
            ((&[5, 1, 8], 6), 16),
            ((&[3, 3, 1, 2, 1, 1, 3, 2, 1], 58), 75),
            ((&[1, 3], 9), 36),
            ((&[1, 1, 3, 3], 74), 576),
        ];

        for ((rank, cars), expected) in test_cases {
            assert_eq!(S::repair_cars(rank.to_vec(), cars), expected);
        }
    }
}
