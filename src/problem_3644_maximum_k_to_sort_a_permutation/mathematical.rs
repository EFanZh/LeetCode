pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut candidate = -1;

        (0..).zip(nums).for_each(|(i, num)| {
            if num != i {
                candidate &= num;
            }
        });

        if candidate == -1 { 0 } else { candidate }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_permutation(nums: Vec<i32>) -> i32 {
        Self::sort_permutation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
