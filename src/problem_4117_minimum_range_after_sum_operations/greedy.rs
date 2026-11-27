pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_range(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a.cast_unsigned();
        let mut b = b.cast_unsigned();
        let mut c = c.cast_unsigned();

        if b < a {
            (a, b) = (b, a);
        }

        if c < b {
            if c < a {
                (a, b, c) = (c, a, b);
            } else {
                (b, c) = (c, b);
            }
        }

        b.min(c - a).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_range(a: i32, b: i32, c: i32) -> i32 {
        Self::min_range(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
