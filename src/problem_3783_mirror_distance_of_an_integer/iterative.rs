pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let n = n.cast_unsigned();
        let mut x = n;
        let mut reversed = 0;

        while x != 0 {
            let digit = x % 10;

            x /= 10;
            reversed = reversed * 10 + digit;
        }

        reversed.abs_diff(n).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn mirror_distance(n: i32) -> i32 {
        Self::mirror_distance(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
