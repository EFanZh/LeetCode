pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn decimal_representation(n: i32) -> Vec<i32> {
        let mut n = n.cast_unsigned();
        let mut buffer = [0; 10];
        let mut i = 10;
        let mut base = 1;

        while n != 0 {
            let digit = n % 10;

            n /= 10;

            if digit != 0 {
                i -= 1;
                buffer[i] = (base * digit).cast_signed();
            }

            base = base.wrapping_mul(10);
        }

        buffer[i..].to_vec()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn decimal_representation(n: i32) -> Vec<i32> {
        Self::decimal_representation(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
