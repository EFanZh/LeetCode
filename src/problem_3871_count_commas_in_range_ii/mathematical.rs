pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let n = n.cast_unsigned();
        let mut result = 0;

        'b_0: {
            'b_1: {
                'b_2: {
                    'b_3: {
                        'b_4: {
                            match n {
                                0..1_000 => break 'b_0,
                                1_000..1_000_000 => break 'b_1,
                                1_000_000..1_000_000_000 => break 'b_2,
                                1_000_000_000..1_000_000_000_000 => break 'b_3,
                                1_000_000_000_000..1_000_000_000_000_000 => break 'b_4,
                                _ => {}
                            }

                            result += n - 999_999_999_999_999;
                        }

                        result += n - 999_999_999_999;
                    }

                    result += n - 999_999_999;
                }

                result += n - 999_999;
            }

            result += n - 999;
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_commas(n: i64) -> i64 {
        Self::count_commas(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
