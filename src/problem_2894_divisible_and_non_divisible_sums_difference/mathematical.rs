pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let n = n as u32;
        let m = NonZeroU32::new(m as _).unwrap();
        let divisible = n / m;
        let divisible_sum = (m.get() + m.get() * divisible) * divisible / 2;
        let total_sum = (1 + n) * n / 2;

        total_sum as i32 - (divisible_sum * 2) as i32
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn difference_of_sums(n: i32, m: i32) -> i32 {
        Self::difference_of_sums(n, m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
