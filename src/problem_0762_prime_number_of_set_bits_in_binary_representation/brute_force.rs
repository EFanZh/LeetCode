pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let left = left as u32;
        let right = right as u32;
        let mut count = 0;

        for x in left..=right {
            if matches!(x.count_ones(), 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19) {
                count += 1;
            }
        }

        count
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        Self::count_prime_set_bits(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
