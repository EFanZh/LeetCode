pub mod connected_components;
pub mod connected_components_2;

pub trait Solution {
    fn remove_stones(stones: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]] as &[_], 5),
            (&[[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]], 3),
            (&[[0, 0]], 0),
        ];

        for (stones, expected) in test_cases {
            assert_eq!(S::remove_stones(stones.iter().map(Vec::from).collect()), expected);
        }
    }
}
