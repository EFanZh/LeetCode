pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    // Number of trailing zeros in (5 * n)!: https://oeis.org/A191610.
    fn f(mut n: u32) -> u32 {
        let mut result = 0;

        while n != 0 {
            result += n;
            n /= 5;
        }

        result
    }

    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k = k as u32;
        let mut left = (k + 1) * 4 / 5;

        loop {
            match Self::f(left).cmp(&k) {
                Ordering::Less => left += 1,
                Ordering::Equal => return 5,
                Ordering::Greater => return 0,
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn preimage_size_fzf(k: i32) -> i32 {
        Self::preimage_size_fzf(k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
