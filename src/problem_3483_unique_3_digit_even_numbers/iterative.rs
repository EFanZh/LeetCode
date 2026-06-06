pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut counts = [0_u8; 10];

        for digit in digits {
            counts[digit.cast_unsigned() as usize] += 1;
        }

        let mut result = 0;

        for i in 1..10 {
            if counts[i] > 0 {
                counts[i] -= 1;

                for j in 0..10 {
                    if counts[j] > 0 {
                        counts[j] -= 1;

                        for k in (0..10).step_by(2) {
                            result += i32::from(counts[k] > 0);
                        }

                        counts[j] += 1;
                    }
                }

                counts[i] += 1;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_numbers(digits: Vec<i32>) -> i32 {
        Self::total_numbers(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
