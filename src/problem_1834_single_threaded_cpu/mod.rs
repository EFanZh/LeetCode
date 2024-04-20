pub mod binary_heap;

pub trait Solution {
    fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 4], [3, 2], [4, 1]] as &[_], &[0, 2, 3, 1] as &[_]),
            (&[[7, 10], [7, 12], [7, 5], [7, 4], [7, 2]], &[4, 3, 2, 0, 1]),
            (
                &[
                    [19, 13],
                    [16, 9],
                    [21, 10],
                    [32, 25],
                    [37, 4],
                    [49, 24],
                    [2, 15],
                    [38, 41],
                    [37, 34],
                    [33, 6],
                    [45, 4],
                    [18, 18],
                    [46, 39],
                    [12, 24],
                ],
                &[6, 1, 2, 9, 4, 10, 0, 11, 5, 13, 3, 8, 12, 7],
            ),
        ];

        for (tasks, expected) in test_cases {
            assert_eq!(S::get_order(tasks.iter().map(Vec::from).collect()), expected);
        }
    }
}
