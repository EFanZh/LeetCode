pub mod layer_by_layer;

pub trait Solution {
    fn generate_matrix(n: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(3, &[&[1, 2, 3] as &[_], &[8, 9, 4], &[7, 6, 5]] as &[_])];

        for (n, expected) in test_cases {
            assert_eq!(S::generate_matrix(n), expected);
        }
    }
}
