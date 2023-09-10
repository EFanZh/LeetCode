pub mod binary_heap;

pub trait Solution {
    fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[1, 2, 3, 4, 5] as &[_], &[5, 2, 3, 3, 3] as &[_]), &[1] as &[_]),
            ((3, &[1, 2, 3, 4], &[1, 2, 1, 2]), &[0]),
            ((3, &[1, 2, 3], &[10, 12, 11]), &[0, 1, 2]),
        ];

        for ((k, arrival, load), expected) in test_cases {
            assert_eq!(S::busiest_servers(k, arrival.to_vec(), load.to_vec()), expected);
        }
    }
}
