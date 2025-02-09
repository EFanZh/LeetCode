pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let ones_1 = num1.count_ones();
        let ones_2 = num2.count_ones();
        let mut probe = 1;

        let min_value = if ones_1 < ones_2 {
            let mut required_zeros = ones_2 - ones_1;

            loop {
                while num1 & probe != 0 {
                    probe <<= 1;
                }

                required_zeros -= 1;
                probe <<= 1;

                if required_zeros == 0 {
                    break;
                }
            }

            !num1
        } else {
            let mut required_ones = ones_1 - ones_2;

            while required_ones != 0 {
                while num1 & probe == 0 {
                    probe <<= 1;
                }

                required_ones -= 1;
                probe <<= 1;
            }

            num1
        };

        num1 ^ (min_value & (probe - 1))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_xor(num1: i32, num2: i32) -> i32 {
        Self::minimize_xor(num1, num2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
