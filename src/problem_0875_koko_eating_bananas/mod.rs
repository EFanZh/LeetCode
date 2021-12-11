pub mod binary_search;

pub trait Solution {
    fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 6, 7, 11] as &[_], 8), 4),
            ((&[30, 11, 23, 4, 20], 5), 30),
            ((&[30, 11, 23, 4, 20], 6), 23),
        ];

        for ((piles, h), expected) in test_cases {
            assert_eq!(S::min_eating_speed(piles.to_vec(), h), expected);
        }
    }
}
