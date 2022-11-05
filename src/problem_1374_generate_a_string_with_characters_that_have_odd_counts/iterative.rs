pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        let mut result = String::with_capacity(n);

        if n % 2 == 0 {
            result.extend(iter::repeat('a').take(n - 1));
            result.push('b');
        } else {
            result.extend(iter::repeat('a').take(n));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn generate_the_string(n: i32) -> String {
        Self::generate_the_string(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
