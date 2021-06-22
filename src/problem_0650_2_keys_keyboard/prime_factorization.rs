pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut n = n as u32;
        let mut result = 0;
        let mut divisor = 2;

        while n != 1 {
            while n % divisor != 0 {
                divisor += 1;
            }

            n /= divisor;
            result += divisor;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_steps(n: i32) -> i32 {
        Self::min_steps(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
