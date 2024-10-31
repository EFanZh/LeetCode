pub mod binary_search_1;
pub mod binary_search_2;

pub trait Solution {
    fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[5, 1, 3] as &[_], &[1, 2, 3, 4, 5] as &[_], 7), &[4, 0, 3] as &[_]),
            ((&[3, 1, 2], &[8, 5, 8], 16), &[2, 0, 2]),
        ];

        for ((spells, potions, success), expected) in test_cases {
            assert_eq!(
                S::successful_pairs(spells.to_vec(), potions.to_vec(), success),
                expected,
            );
        }
    }
}
