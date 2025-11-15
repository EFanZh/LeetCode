pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let mut iter = nums.iter().copied();

        iter.next().is_none_or(|mut prev| {
            iter.all(|num| {
                let result = (num ^ prev) & 1 != 0;

                prev = num;

                result
            })
        })
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_array_special(nums: Vec<i32>) -> bool {
        Self::is_array_special(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
