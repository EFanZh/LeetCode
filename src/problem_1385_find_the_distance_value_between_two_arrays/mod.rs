pub mod binary_search;
pub mod buckets;

pub trait Solution {
    fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 5, 8] as &[_], &[10, 9, 1, 8] as &[_], 2), 2),
            ((&[1, 4, 2, 3], &[-4, -3, 6, 10, 20, 30], 3), 2),
            ((&[2, 1, 100, 3], &[-5, -2, 10, -3, 7], 6), 1),
        ];

        for ((arr1, arr2, d), expected) in test_cases {
            assert_eq!(S::find_the_distance_value(arr1.to_vec(), arr2.to_vec(), d), expected,);
        }
    }
}
