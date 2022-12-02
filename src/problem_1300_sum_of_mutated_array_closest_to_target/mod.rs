pub mod iterative;

pub trait Solution {
    fn find_best_value(arr: Vec<i32>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 9, 3] as &[_], 10), 3),
            ((&[2, 3, 5], 10), 5),
            ((&[60864, 25176, 27249, 21296, 20204], 56803), 11361),
            ((&[2, 3, 5], 11), 5),
        ];

        for ((arr, target), expected) in test_cases {
            assert_eq!(S::find_best_value(arr.to_vec(), target), expected);
        }
    }
}
