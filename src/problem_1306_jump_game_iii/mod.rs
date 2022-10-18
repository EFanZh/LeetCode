pub mod bfs;

pub trait Solution {
    fn can_reach(arr: Vec<i32>, start: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[4, 2, 3, 0, 3, 1, 2] as &[_], 5), true),
            ((&[4, 2, 3, 0, 3, 1, 2], 0), true),
            ((&[3, 0, 2, 1, 2], 2), false),
            ((&[0], 0), true),
        ];

        for ((arr, start), expected) in test_cases {
            assert_eq!(S::can_reach(arr.to_vec(), start), expected);
        }
    }
}
