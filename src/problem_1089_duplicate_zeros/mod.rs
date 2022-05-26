pub mod count_zeros;

pub trait Solution {
    fn duplicate_zeros(arr: &mut Vec<i32>);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 0, 2, 3, 0, 4, 5, 0] as &[_], &[1, 0, 0, 2, 3, 0, 0, 4] as &[_]),
            (&[1, 2, 3], &[1, 2, 3]),
            (&[0, 0, 0, 0, 0, 0, 0], &[0, 0, 0, 0, 0, 0, 0]),
        ];

        for (arr, expected) in test_cases {
            let mut arr = arr.to_vec();

            S::duplicate_zeros(&mut arr);

            assert_eq!(arr, expected);
        }
    }
}
