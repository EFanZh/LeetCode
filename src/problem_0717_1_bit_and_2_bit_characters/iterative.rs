pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut iter = bits.iter().rev().copied();

        iter.next().unwrap();

        let ones = iter.take_while(|&b| b != 0).count();

        ones % 2 == 0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_one_bit_character(bits: Vec<i32>) -> bool {
        Self::is_one_bit_character(bits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
