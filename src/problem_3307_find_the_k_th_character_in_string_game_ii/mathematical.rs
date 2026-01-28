pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut offset = 0;
        let mut i = k.cast_unsigned() - 1;

        while i != 0 {
            offset += operations
                .get(i.trailing_zeros() as usize)
                .copied()
                .map_or(0, i32::cast_unsigned);

            i &= i - 1;
        }

        char::try_from(u32::from('a') + offset % 26).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_character(k: i64, operations: Vec<i32>) -> char {
        Self::kth_character(k, operations)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
