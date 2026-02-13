pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZero;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        let mut n = n.cast_unsigned();
        let t = NonZero::new(t.cast_unsigned()).unwrap();

        while {
            let mut product = 1;
            let mut x = n;

            while x != 0 {
                product *= x % 10;
                x /= 10;
            }

            product % t != 0
        } {
            n += 1;
        }

        n.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_number(n: i32, t: i32) -> i32 {
        Self::smallest_number(n, t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
