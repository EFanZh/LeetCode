pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut n = n as u8;
        let k = NonZeroU8::new(k as _).unwrap();
        let mut result = 0;

        while n != 0 {
            result += n % k;
            n = n / k;
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sum_base(n: i32, k: i32) -> i32 {
        Self::sum_base(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
