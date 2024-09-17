pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn evaluate(s: &[u8]) -> u16 {
        let (&first, rest) = s.split_first().unwrap();
        let mut result = 0;
        let mut term = u16::from(first - b'0');

        for window in rest.chunks_exact(2) {
            let [operator, rhs]: [_; 2] = window.try_into().ok().unwrap();
            let rhs = u16::from(rhs) - u16::from(b'0');

            if operator == b'+' {
                result += term;
                term = rhs;
            } else {
                term *= rhs;
            }
        }

        result + term
    }

    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let s = s.as_bytes();
        let n = s.len() / 2 + 1;
        let correct_answer = Self::evaluate(s);
        let mut cache = vec![Vec::new(); n * n];
        let mut values_set = [false; 1001];

        for (&c, values) in s.iter().step_by(2).zip(&mut cache) {
            values.push(u16::from(c - b'0'));
        }

        for operators in 1..n {
            for start in 0..n - operators {
                let mut values = Vec::new();

                for left_operators in 0..operators {
                    let left = &*cache[n * left_operators + start];
                    let right = &*cache[n * (operators - 1 - left_operators) + start + left_operators + 1];

                    if s[(start + left_operators) * 2 + 1] == b'+' {
                        for &lhs in left {
                            for &rhs in right {
                                let x = lhs + rhs;

                                if let Some(state @ false) = values_set.get_mut(usize::from(x)) {
                                    *state = true;
                                    values.push(x);
                                }
                            }
                        }
                    } else {
                        for &lhs in left {
                            for &rhs in right {
                                if let Some(x) = lhs.checked_mul(rhs) {
                                    if let Some(state @ false) = values_set.get_mut(usize::from(x)) {
                                        *state = true;
                                        values.push(x);
                                    }
                                }
                            }
                        }
                    }
                }

                for &value in &values {
                    values_set[usize::from(value)] = false;
                }

                cache[n * operators + start] = values;
            }
        }

        for &value in &cache[n * (n - 1)] {
            values_set[usize::from(value)] = true;
        }

        drop(cache);

        answers
            .iter()
            .map(|&answer| {
                let answer = answer as u16;

                if answer == correct_answer {
                    5
                } else if values_set[usize::from(answer)] {
                    2
                } else {
                    0
                }
            })
            .sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        Self::score_of_students(s, answers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
