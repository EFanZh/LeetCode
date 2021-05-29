pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::iter;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut visited = iter::once(n).collect::<HashSet<_>>();

        loop {
            let mut next = 0;

            while n != 0 {
                let digit = n % 10;

                next += digit * digit;
                n /= 10;
            }

            if visited.insert(next) {
                n = next;
            } else {
                return next == 1;
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_happy(n: i32) -> bool {
        Self::is_happy(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
