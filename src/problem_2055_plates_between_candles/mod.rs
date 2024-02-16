pub mod prefix_sum;

pub trait Solution {
    fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("**|**|***|", &[[2, 5], [5, 9]] as &[_]), &[2, 3] as &[_]),
            (
                ("***|**|*****|**||**|*", &[[1, 17], [4, 5], [14, 17], [5, 11], [15, 16]]),
                &[9, 0, 0, 0, 0],
            ),
        ];

        for ((s, queries), expected) in test_cases {
            assert_eq!(
                S::plates_between_candles(s.to_string(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
