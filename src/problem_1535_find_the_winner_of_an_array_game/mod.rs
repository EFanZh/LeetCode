pub mod simulation;

pub trait Solution {
    fn get_winner(arr: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[2, 1, 3, 5, 4, 6, 7] as &[_], 2), 5), ((&[3, 2, 1], 10), 3)];

        for ((arr, k), expected) in test_cases {
            assert_eq!(S::get_winner(arr.to_vec(), k), expected);
        }
    }
}
