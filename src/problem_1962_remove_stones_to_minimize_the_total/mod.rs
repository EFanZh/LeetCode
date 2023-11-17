pub mod binary_heap;

pub trait Solution {
    fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[5, 4, 9] as &[_], 2), 12), ((&[4, 3, 6, 7], 3), 12)];

        for ((piles, k), expected) in test_cases {
            assert_eq!(S::min_stone_sum(piles.to_vec(), k), expected);
        }
    }
}
