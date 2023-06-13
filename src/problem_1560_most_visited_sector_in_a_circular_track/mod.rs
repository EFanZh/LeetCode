pub mod mathematical;

pub trait Solution {
    fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[1, 3, 1, 2] as &[_]), &[1, 2] as &[_]),
            ((2, &[2, 1, 2, 1, 2, 1, 2, 1, 2]), &[2]),
            ((7, &[1, 3, 5, 7]), &[1, 2, 3, 4, 5, 6, 7]),
            ((3, &[3, 2, 1, 2, 1, 3, 2, 1, 2, 1, 3, 2, 3, 1]), &[1, 3]),
        ];

        for ((n, rounds), expected) in test_cases {
            assert_eq!(S::most_visited(n, rounds.to_vec()), expected);
        }
    }
}
