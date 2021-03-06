pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = vec![0; 1 << n];

        for bits in 0..n {
            let one_bit = 1 << bits;
            let (old_slice, new_slice) = result.split_at_mut(one_bit as _);

            for (new_num, old_num) in new_slice.iter_mut().zip(old_slice.iter().rev()) {
                *new_num = one_bit | old_num;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        Self::gray_code(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
