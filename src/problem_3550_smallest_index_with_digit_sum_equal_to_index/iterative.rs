pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        (0..)
            .zip(&nums)
            .find_map(|(i, num)| {
                let mut x = num.cast_unsigned();
                let mut sum = 0;

                while x != 0 {
                    sum += x % 10;
                    x /= 10;
                }

                (sum == i).then_some(i)
            })
            .map_or(-1, u32::cast_signed)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_index(nums: Vec<i32>) -> i32 {
        Self::smallest_index(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
