pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        let mut final_sum = final_sum as u64;
        let mut result = Vec::new();

        if final_sum % 2 == 0 {
            let mut candidate = 2;

            loop {
                final_sum -= candidate;

                if candidate < final_sum {
                    result.push(candidate as _);

                    candidate += 2;
                } else {
                    candidate += final_sum;
                    result.push(candidate as _);

                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        Self::maximum_even_split(final_sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
