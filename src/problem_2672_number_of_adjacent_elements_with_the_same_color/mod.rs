pub mod map;

pub trait Solution {
    fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (4, &[[0, 2], [1, 2], [3, 1], [1, 1], [2, 1]] as &[_]),
                &[0, 1, 1, 0, 2] as &[_],
            ),
            ((1, &[[0, 100_000]]), &[0]),
        ];

        for ((n, queries), expected) in test_cases {
            assert_eq!(S::color_the_array(n, queries.iter().map(Vec::from).collect()), expected);
        }
    }
}
