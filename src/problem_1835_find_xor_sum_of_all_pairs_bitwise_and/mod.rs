pub mod mathematical;

pub trait Solution {
    fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((&[1, 2, 3] as &[_], &[6, 5] as &[_]), 0), ((&[12], &[4]), 4)];

        for ((arr1, arr2), expected) in test_cases {
            assert_eq!(S::get_xor_sum(arr1.to_vec(), arr2.to_vec()), expected);
        }
    }
}
