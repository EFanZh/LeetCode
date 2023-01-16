pub mod greedy;

pub trait Solution {
    fn longest_diverse_string(a: i32, b: i32, c: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 1, 7), 8), ((7, 1, 0), 5), ((7, 1, 0), 5), ((2, 2, 1), 5)];

        for ((a, b, c), expected) in test_cases {
            let result = S::longest_diverse_string(a, b, c);

            assert_eq!(result.len(), expected);

            let mut count_a = 0;
            let mut count_b = 0;
            let mut count_c = 0;

            for c in result.bytes() {
                match c {
                    b'a' => count_a += 1,
                    b'b' => count_b += 1,
                    _ => {
                        assert_eq!(c, b'c');

                        count_c += 1;
                    }
                }
            }

            assert!(count_a <= a);
            assert!(count_b <= b);
            assert!(count_c <= c);

            let mut prev_1 = 0;
            let mut prev_2 = 0;

            for c in result.bytes() {
                assert!(prev_1 != prev_2 || prev_2 != c);

                prev_1 = prev_2;
                prev_2 = c;
            }
        }
    }
}
