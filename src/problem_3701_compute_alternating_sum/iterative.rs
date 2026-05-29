pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn alternating_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let (chunks, rest) = nums.as_chunks::<2>();

        for &[add, minus] in chunks {
            result += add;
            result -= minus;
        }

        if let &[last] = rest {
            result += last;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn alternating_sum(nums: Vec<i32>) -> i32 {
        Self::alternating_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
