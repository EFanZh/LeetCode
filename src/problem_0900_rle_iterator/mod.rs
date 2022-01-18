pub mod iterative;

pub trait RLEIterator {
    fn new(encoding: Vec<i32>) -> Self;
    fn next(&mut self, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::RLEIterator;

    pub fn run<I: RLEIterator>() {
        let test_cases = vec![(
            &[3, 8, 0, 9, 2, 5] as &[_],
            (&[(2, 8), (1, 8), (1, 5), (2, -1)] as &[_]),
        )];

        for (encoding, nexts) in test_cases {
            let mut rle_iterator = I::new(encoding.to_vec());

            for &(n, expected) in nexts {
                assert_eq!(rle_iterator.next(n), expected);
            }
        }
    }
}
