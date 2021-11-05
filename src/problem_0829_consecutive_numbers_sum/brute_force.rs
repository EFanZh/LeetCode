pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let n = n as u32;
        let mut result = 1;
        let mut size = 2;
        let max_size = (f64::sqrt(f64::from(n * 2) + 0.25) + 0.5).ceil() as u32;

        loop {
            if size < max_size {
                if n % size == size / 2 {
                    result += 1;
                }

                size += 1;
            } else {
                break;
            }

            if size < max_size {
                if n % size == 0 {
                    result += 1;
                }

                size += 1;
            } else {
                break;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn consecutive_numbers_sum(n: i32) -> i32 {
        Self::consecutive_numbers_sum(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
