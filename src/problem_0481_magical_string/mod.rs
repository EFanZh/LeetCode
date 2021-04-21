pub mod deque;
pub mod generator;
pub mod state_machine;
pub mod state_machine_2;

pub trait Solution {
    fn magical_string(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        // https://oeis.org/A156077.
        let test_cases = [
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (5, 3),
            (6, 3),
            (7, 4),
            (8, 4),
            (9, 4),
            (10, 5),
            (100_000, 49972),
        ];

        for (n, expected) in test_cases.iter().copied() {
            assert_eq!(S::magical_string(n), expected);
        }
    }
}
