pub mod binary_heap;

pub trait Solution {
    fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[25, 64, 9, 4, 100] as &[_], 4), 29), ((&[1, 1, 1, 1], 4), 4)];

        for ((gifts, k), expected) in test_cases {
            assert_eq!(S::pick_gifts(gifts.to_vec(), k), expected);
        }
    }
}
