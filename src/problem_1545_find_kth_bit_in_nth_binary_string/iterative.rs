pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        let _ = n;
        let mut k = k as u32;
        let mut inverted = false;

        while k & (k - 1) != 0 {
            k = (1 << (32 - k.leading_zeros())) - k;
            inverted = !inverted;
        }

        if (k == 1) == inverted {
            '1'
        } else {
            '0'
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_kth_bit(n: i32, k: i32) -> char {
        Self::find_kth_bit(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
