pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn gcd(mut lhs: u32, mut rhs: u32) -> u32 {
        while rhs != 0 {
            let new_rhs = lhs % rhs;

            lhs = rhs;
            rhs = new_rhs;
        }

        lhs
    }

    pub fn is_good_array(nums: Vec<i32>) -> bool {
        let mut iter = nums.into_iter().map(i32::cast_unsigned);
        let mut gcd = iter.next().unwrap();

        for num in iter {
            gcd = Self::gcd(gcd, num);
        }

        gcd == 1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_good_array(nums: Vec<i32>) -> bool {
        Self::is_good_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
