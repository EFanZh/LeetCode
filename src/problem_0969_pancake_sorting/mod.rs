pub mod iterative;

pub trait Solution {
    fn pancake_sort(arr: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [&[3, 2, 4, 1] as &[_], &[1, 2, 3]];

        for arr in test_cases {
            let result = S::pancake_sort(arr.to_vec());
            let mut arr = arr.to_vec();

            assert!(result.len() <= arr.len() * 10);

            for x in result {
                arr[..x as usize].reverse();
            }

            for (expected, actual) in (1..).zip(arr) {
                assert_eq!(actual, expected);
            }
        }
    }
}
