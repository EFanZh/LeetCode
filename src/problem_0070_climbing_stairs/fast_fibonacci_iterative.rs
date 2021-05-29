pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

// https://en.wikipedia.org/wiki/Fibonacci_number#Matrix_form.

impl Solution {
    fn fast_fibonacci(n: i32) -> i32 {
        let n = n as u32;
        let bits = (mem::size_of_val(&n) as u32) * 8 - n.leading_zeros();
        let mut matrix = (1, 0);

        for bit in (0..bits).rev() {
            let (a, b) = matrix;
            let c = a + b;

            matrix = if n & (1 << bit) == 0 {
                (a * a + b * b, (a + c) * b)
            } else {
                ((a + c) * b, b * b + c * c)
            };
        }

        matrix.1
    }

    pub fn climb_stairs(n: i32) -> i32 {
        Self::fast_fibonacci(n + 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn climb_stairs(n: i32) -> i32 {
        Self::climb_stairs(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
