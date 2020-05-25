pub struct Solution {}

// https://en.wikipedia.org/wiki/Fibonacci_number#Matrix_form.

impl Solution {
    fn fib_helper(n: i32) -> (i32, i32) {
        if n == 0 {
            (1, 0)
        } else {
            let (a, b) = Self::fib_helper(n / 2);
            let c = a + b;

            if n % 2 == 0 {
                (a * a + b * b, (a + c) * b)
            } else {
                ((a + c) * b, b * b + c * c)
            }
        }
    }

    pub fn fib(n: i32) -> i32 {
        Self::fib_helper(n).1
    }
}

impl super::Solution for Solution {
    fn fib(n: i32) -> i32 {
        Self::fib(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
