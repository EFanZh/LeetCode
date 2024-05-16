pub mod map;

pub trait Solution {
    fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[15, 88] as &[_], &[&[88] as &[_], &[15]] as &[&[_]]), true),
            ((&[49, 18, 16], &[&[16, 18, 49]]), false),
            ((&[91, 4, 64, 78], &[&[78], &[4, 64], &[91]]), true),
            (
                (&[91, 2, 4, 64, 5, 78, 12, 9], &[&[78, 12, 3], &[4, 64, 5], &[91, 2]]),
                false,
            ),
        ];

        for ((arr, pieces), expected) in test_cases {
            assert_eq!(
                S::can_form_array(arr.to_vec(), pieces.iter().copied().map(<[_]>::to_vec).collect()),
                expected,
            );
        }
    }
}
