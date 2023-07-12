pub mod hash_map;

pub trait Solution {
    fn count_pairs(deliciousness: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 5, 7, 9] as &[_], 4), (&[1, 1, 1, 3, 3, 3, 7], 15), (&[0], 0)];

        for (deliciousness, expected) in test_cases {
            assert_eq!(S::count_pairs(deliciousness.to_vec()), expected);
        }
    }
}
