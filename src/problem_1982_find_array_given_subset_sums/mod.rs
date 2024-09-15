pub mod divide_and_conquer;

pub trait Solution {
    fn recover_array(n: i32, sums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (3, &[-3, -2, -1, 0, 0, 1, 2, 3] as &[_]),
            (2, &[0, 0, 0, 0]),
            (4, &[0, 0, 5, 5, 4, -1, 4, 9, 9, -1, 4, 3, 4, 8, 3, 8]),
            (
                4,
                &[
                    305, -76, -381, 0, -457, -183, -762, -381, 503, 198, 884, 579, 198, 122, -76, 503,
                ],
            ),
        ];

        let mut buffer = Vec::new();

        for (n, sums) in test_cases {
            let result = S::recover_array(n, sums.to_vec());

            buffer.push(0);

            for value in result {
                for i in 0..buffer.len() {
                    let new_value = buffer[i] + value;

                    buffer.push(new_value);
                }
            }

            assert_eq!(
                test_utilities::unstable_sorted(buffer.iter().copied()),
                test_utilities::unstable_sorted(sums.iter().copied()),
            );

            buffer.clear();
        }
    }
}
