pub mod binary_search;
pub mod binary_search_2;

pub trait Solution {
    fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 10, 3, 10, 2] as &[_], 3, 1), 3),
            ((&[1, 10, 3, 10, 2], 3, 2), -1),
            ((&[7, 7, 7, 7, 12, 7, 7], 2, 3), 12),
            (
                (
                    &[62, 75, 98, 63, 47, 65, 51, 87, 22, 27, 73, 92, 76, 44, 13, 90, 100, 85],
                    2,
                    7,
                ),
                98,
            ),
            ((&[97, 83], 2, 1), 97),
            ((&[83, 97], 2, 1), 97),
        ];

        for ((bloom_day, m, k), expected) in test_cases {
            assert_eq!(S::min_days(bloom_day.to_vec(), m, k), expected);
        }
    }
}
