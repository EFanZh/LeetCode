pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn largest_integer(n: i32, s: i32) -> i32 {
        let mut n = n.cast_unsigned();
        let mut s = s.cast_unsigned();

        if 9 * n < s {
            -1
        } else {
            let nines = s / 9;
            let mut result = u32::pow(10, nines) - 1;

            s %= 9;
            n -= nines;

            if s != 0 {
                result = result * 10 + s;
                n -= 1;
            }

            (result * u32::pow(10, n)).cast_signed()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn largest_integer(n: i32, s: i32) -> i32 {
        Self::largest_integer(n, s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
