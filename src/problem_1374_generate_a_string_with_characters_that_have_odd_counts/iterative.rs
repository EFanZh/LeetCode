pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let n = n as usize;
        let mut result = String::with_capacity(n);

        if n.is_multiple_of(2) {
            result.extend(iter::repeat_n('a', n - 1));
            result.push('b');
        } else {
            result.extend(iter::repeat_n('a', n));
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
