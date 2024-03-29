pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);

        while b != 0 {
            let sum = a ^ b;
            let carry = (a & b) << 1;

            a = sum;
            b = carry;
        }

        a
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_sum(a: i32, b: i32) -> i32 {
        Self::get_sum(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
